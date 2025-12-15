use std::sync::LazyLock;
use std::sync::Mutex;
use tauri_plugin_dialog::FilePath;
use crate::{job_manager, storage_manager};
use crate::storage_manager::to_json_string;

static RECOVERY_PROGRESS: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(-1.0));
static RECOVERY_LOGS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));

pub fn verify_recovery_file(file_path: &str) -> &str {
    let file_path = file_path.replace("\\", "/");

    println!("Verifying recovery file at path: {}", file_path);
    // Replace all backslashes with forward slashes for consistency
    return match storage_manager::read_json_file::<Vec<Vec<String>>>(
        file_path.to_string()
    ) {
        Ok(recovery_data) => {
            // Check that all inner arrays have exactly two string elements
            for entry in recovery_data.iter() {
                if entry.len() != 2 {
                    return "Error: Each entry in the recovery file must contain exactly two string elements.";
                }
            }

            "Valid"
        }
        Err(e) => {
            println!("Error reading recovery file: {}", e);
            "Error: Recovery file is not JSON or not in the correct format."
        },
    }
}


pub fn get_recovery_file(file_path: &str) -> String {
    let file_path = file_path.replace("\\", "/");

    println!("Verifying recovery file at path: {}", file_path);
    // Replace all backslashes with forward slashes for consistency
    return match storage_manager::read_json_file::<Vec<Vec<String>>>(
        file_path.to_string()
    ) {
        Ok(recovery_data) => {
            // Check that all inner arrays have exactly two string elements
            for entry in recovery_data.iter() {
                if entry.len() != 2 {
                    return "Error: Each entry in the recovery file must contain exactly two string elements.".to_string();
                }
            }

            to_json_string(&recovery_data)
        }
        Err(e) => {
            println!("Error reading recovery file: {}", e);
            "Error: Recovery file is not JSON or not in the correct format.".to_string()
        },
    }
}

fn get_last_modified_time(path: &std::path::Path) -> Option<std::time::SystemTime> {
    match std::fs::metadata(path) {
        Ok(metadata) => metadata.modified().ok(),
        Err(_) => None,
    }
}

pub fn run_recovery(file_path: &str, recovery_mode: &str) -> bool {
    let recovery_data_json = get_recovery_file(file_path);
    let job_statuses = job_manager::get_all_job_statuses();
    if (!job_statuses.is_empty()) {
        println!("Cannot run recovery while jobs are in progress.");
        return false;
    }

    let recovery_data: Vec<Vec<String>> = match storage_manager::from_json_string(recovery_data_json) {
        Ok(data) => data,
        Err(e) => {
            println!("Error parsing recovery data: {}", e);
            return false;
        }
    };

    tauri::async_runtime::spawn(recovery_worker(recovery_data, recovery_mode.to_string()));

    true
}

async fn recovery_worker(recovery_data: Vec<Vec<String>>, recovery_mode: String) {
    RECOVERY_LOGS.lock().unwrap().clear();
    
    let mut file_index = 0;
    for entry in recovery_data.iter() {
        file_index += 1;

        if (file_index % 10 == 0) {
            println!("Recovery progress: {}/{}", file_index, recovery_data.len());

            let progress = (file_index as f32 / recovery_data.len() as f32);
            *RECOVERY_PROGRESS.lock().unwrap() = progress;
        }

        let original_file_name = entry[0].clone();
        let destination_file_name = entry[1].clone();

        let original_file = std::path::Path::new(&original_file_name);
        let destination_file = std::path::Path::new(&destination_file_name);

        if (!original_file.exists()){
            println!("Original file does not exist: {}", original_file_name);
            RECOVERY_LOGS.lock().unwrap().push(format!("Could not recover {}, Original file does not exist: {}", destination_file_name, original_file_name));
            continue;
        }

        if (!destination_file.exists()){
            // Ensure parent directory exists, then create the file
            if let Some(parent) = destination_file.parent() {
                if !parent.exists() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        println!("Error creating directory {}: {}", parent.display(), e);
                        RECOVERY_LOGS.lock().unwrap().push(format!(
                            "Could not recover {}, Error creating directory {}: {}",
                            destination_file_name,
                            parent.display(),
                            e
                        ));
                        continue;
                    }
                }
            }

            println!("Destination file does not exist, creating new file: {}", destination_file_name);
            // Copy the original file to the destination
            match std::fs::copy(&original_file, &destination_file) {
                Ok(_) => {
                    println!("Successfully recovered file: {}", destination_file_name);
                }
                Err(e) => {
                    println!("Error copying file to {}: {}", destination_file_name, e);
                    RECOVERY_LOGS.lock().unwrap().push(format!("Could not recover {}, Error copying file to {}: {}", destination_file_name, destination_file_name, e));
                }
            }

            continue;
        }

        if recovery_mode == "Skip Existing Files" {
            println!("Skipping existing file: {}", destination_file_name);
            continue;
        }

        if recovery_mode == "Overwrite Existing Files" {
            println!("Overwriting existing file: {}", destination_file_name);
            // Copy the original file to the destination, overwriting it
            match std::fs::copy(&original_file, &destination_file) {
                Ok(_) => {
                    println!("Successfully recovered file: {}", destination_file_name);
                }
                Err(e) => {
                    println!("Error copying file to {}: {}", destination_file_name, e);
                    RECOVERY_LOGS.lock().unwrap().push(format!("Could not recover {}, Error copying file to {}: {}", destination_file_name, destination_file_name, e));
                }
            }
        }

        if recovery_mode == "Keep Most Recently Updated Files" {
            let original_modified = get_last_modified_time(&original_file);
            let destination_modified = get_last_modified_time(&destination_file);

            if (original_modified > destination_modified) {
                println!("Original file is more recently updated. Overwriting: {}", destination_file_name);
                // Copy the original file to the destination, overwriting it
                match std::fs::copy(&original_file, &destination_file) {
                    Ok(_) => {
                        println!("Successfully recovered file: {}", destination_file_name);
                    }
                    Err(e) => {
                        println!("Error copying file to {}: {}", destination_file_name, e);
                        RECOVERY_LOGS.lock().unwrap().push(format!("Could not recover {}, Error copying file to {}: {}", destination_file_name, destination_file_name, e));
                    }
                }
            } else {
                println!("Destination file is more recently updated. Skipping: {}", destination_file_name);
            }
        }
    }
    *RECOVERY_PROGRESS.lock().unwrap() = 1.0;
}

pub fn get_recovery_progress() -> f32 {
    RECOVERY_PROGRESS.lock().unwrap().clone()
}

pub fn get_recovery_logs() -> Vec<String> {
    RECOVERY_LOGS.lock().unwrap().clone()
}

pub fn clear_recovery_status(){
    *RECOVERY_PROGRESS.lock().unwrap() = -1.0;
    RECOVERY_LOGS.lock().unwrap().clear();
}