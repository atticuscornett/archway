use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::{drive_manager, storage_manager};
use crate::structs::{JobInfo, JobStatus};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

static JOB_STATUSES: Lazy<Mutex<Vec<JobStatus>>> = Lazy::new(|| Mutex::new(Vec::new()));

/*
Job Steps:
1. Indexing files to move
2. Copying files
3. Verifying files
4. (If moving) Deleting original files
 */

pub fn start_job(uuid: String) -> bool {
    let already_running = {
        let job_statuses = JOB_STATUSES.lock().unwrap();
        job_statuses.iter().any(|js| js.job.uuid == uuid && !js.completed)
    };

    if already_running {
        println!("Job with UUID {} is already running.", uuid);
        return false;
    }

    clear_job(&uuid);


    let new_job = storage_manager::get_job_by_uuid(&uuid);

    let new_job_status = JobStatus {
        job: new_job.clone(),
        step: 0,
        total_steps: if new_job.clone().file_behavior == "copy" { 3 } else { 4 }, // If copying, skip the deletion step
        step_title: String::from("Initializing Job"),
        last_action: String::from("Starting job..."),
        success: true,
        completed: false,
        percent: 0.0
    };

    JOB_STATUSES.lock().unwrap().push(new_job_status);

    tauri::async_runtime::spawn(job_stage_one(uuid));

    true

}

pub fn clear_job(uuid: &str) -> bool {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    if let Some(pos) = job_statuses.iter().position(|js| js.job.uuid == uuid) {
        job_statuses.remove(pos);
        return true;
    }
    false
}

pub fn get_all_job_statuses() -> Vec<JobStatus> {
    JOB_STATUSES.lock().unwrap().clone()
}

fn update_job_status(uuid: &str, step: u32, step_title: String, last_action: String, success: bool, completed: bool, percent: f32) {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    if let Some(job_status) = job_statuses.iter_mut().find(|js| js.job.uuid == uuid) {
        job_status.step = step;
        job_status.step_title = step_title;
        job_status.last_action = last_action;
        job_status.success = success;
        job_status.completed = completed;
        job_status.percent = percent;
    }
}

fn update_last_action(uuid: &str, last_action: String) {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    if let Some(job_status) = job_statuses.iter_mut().find(|js| js.job.uuid == uuid) {
        job_status.last_action = last_action;
    }
}

pub fn clear_completed_jobs() {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    job_statuses.retain(|js| !js.completed);
}

fn update_job_progress(uuid: &str, percent: f32) {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    if let Some(job_status) = job_statuses.iter_mut().find(|js| js.job.uuid == uuid) {
        job_status.percent = percent;
    }
}

fn get_all_subfolders(path: &str) -> Vec<String> {
    let mut subfolders = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                let subfolder_path = entry.path().to_string_lossy().to_string();
                subfolders.push(subfolder_path.clone());
                subfolders.extend(get_all_subfolders(&subfolder_path));
            }
        }
    }
    subfolders
}

fn get_all_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                files.push(entry.path().to_string_lossy().to_string());
            } else if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                files.extend(get_all_files(&entry.path().to_string_lossy()));
            }
        }
    }
    files
}

fn get_last_access_time(path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    let atime = metadata.accessed()?;
    let duration = atime.duration_since(UNIX_EPOCH).unwrap();
    Ok(duration.as_secs())
}

fn check_older_than(time: u64, period: &str) -> bool{
    let now = SystemTime::now();
    let period_duration = match period {
        "week" => 7 * 24 * 60 * 60, // 1 week in seconds
        "2weeks" => 14 * 24 * 60 * 60, // 2 weeks in seconds
        "month" => 30 * 24 * 60 * 60, // 1 month in seconds
        "2months" => 60 * 24 * 60 * 60, // 2 months in seconds
        "3months" => 90 * 24 * 60 * 60, // 3 months in seconds
        "6months" => 180 * 24 * 60 * 60, // 6 months in seconds
        "year" => 365 * 24 * 60 * 60, // 1 year in seconds
        _ => 999999999999999, // Invalid period
    };
    
    if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
        if duration.as_secs() > time + period_duration {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn get_file_size(path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

async fn job_stage_one(uuid: String) {
    update_job_status(uuid.as_str(), 1, String::from("Indexing Files"),
                      String::from("Getting all input folders..."), true, false, -1.0);

    let input_dirs = storage_manager::get_job_by_uuid(&uuid).input_dirs;
    let mut all_folders: Vec<String> = Vec::new();
    for input_dir in input_dirs {
        if (input_dir.path_type == "library") {
            if (input_dir.path == "documents") {
                let documents_path = dirs::document_dir().unwrap();
                all_folders.push(documents_path.to_string_lossy().to_string());
            } else if (input_dir.path == "downloads") {
                let downloads_path = dirs::download_dir().unwrap();
                all_folders.push(downloads_path.to_string_lossy().to_string());
            } else if (input_dir.path == "desktop") {
                let desktop_path = dirs::desktop_dir().unwrap();
                all_folders.push(desktop_path.to_string_lossy().to_string());
            } else if (input_dir.path == "music") {
                let music_path = dirs::audio_dir().unwrap();
                all_folders.push(music_path.to_string_lossy().to_string());
            } else if (input_dir.path == "pictures") {
                let pictures_path = dirs::picture_dir().unwrap();
                all_folders.push(pictures_path.to_string_lossy().to_string());
            } else if (input_dir.path == "videos") {
                let videos_path = dirs::video_dir().unwrap();
                all_folders.push(videos_path.to_string_lossy().to_string());
            } else {
                println!("Unknown library path: {}", input_dir.path);
            }
        }
        all_folders.push(input_dir.path.clone());
        all_folders.extend(get_all_subfolders(&input_dir.path));
    }

    update_last_action(uuid.as_str(), String::from("Getting all files..."));
    update_job_progress(uuid.as_str(), 0.33);
    let mut all_files: Vec<String> = Vec::new();
    for input_dir in all_folders.iter() {
        all_files.extend(get_all_files(input_dir.as_str()));
    }

    update_last_action(uuid.as_str(), String::from("Applying filters..."));
    update_job_progress(uuid.as_str(), 0.66);
    let filters = storage_manager::get_job_by_uuid(&uuid).file_filters;
    for filter in filters {
        if filter.filter_type == "extension" {
            let mut allowed_extensions = filter.traits.extensions.unwrap();
            for extension in allowed_extensions.clone() {
                if (extension == "documents:special"){
                    allowed_extensions.extend(vec![String::from("doc"), String::from("docx"),
                    String::from("pdf"), String::from("txt"), String::from("odt"), String::from("rtf"),
                        String::from("md"), String::from("epub"), String::from("pptx"), String::from("xls"), String::from("xlsx")]);
                }

            }

            all_files.retain(|file| {
                let file_extension = file.split('.').last().unwrap_or("");
                allowed_extensions.contains(&file_extension.to_lowercase())
            });
        }
        if (filter.filter_type == "size") {
            let threshold = filter.traits.size.unwrap();


            all_files.retain(|file| {
                match get_file_size(file.as_str()) {
                    Ok(size) => (size/1000)/1000 >= threshold,
                    Err(_) => {
                        println!("Could not get size for file: {}", file);
                        false
                    }, // If we can't get the size, exclude the file
                }
            });
        }
        if (filter.filter_type == "last-used") {
            let threshold = filter.traits.lastused.unwrap();
            all_files.retain(|file| {
                match get_last_access_time(file.as_str()) {
                    Ok(last_accessed) => {
                        check_older_than(last_accessed, threshold.as_str())
                    }
                    Err(_) => { 
                        println!("Could not get last accessed time for file: {}", file);
                        false
                    }, // If we can't get the last accessed time, exclude the file
                }
            });
        }
    }


    println!("All folders to move: {:?}", all_folders);
    println!("All files to move: {:?}", all_files);

    tauri::async_runtime::spawn(job_stage_two(uuid, all_files));

}

async fn job_stage_two(uuid: String, files: Vec<String>) {
    update_job_status(uuid.as_str(), 2, String::from("Copying Files"),
                      String::from("Starting to copy files..."), true, false, -1.0);

    let job_info = storage_manager::get_job_by_uuid(&uuid);

    let output_device = job_info.output_device.clone();

    let output_dir = job_info.output_dir.clone();
    let drive = drive_manager::get_root_drive(output_dir.as_str()).unwrap();
    let drive_uuid = drive_manager::get_drive_uuid(drive.as_str());

    if !std::path::Path::new(&drive).exists() {
        println!("Drive does not exist: {}", drive);
        update_job_status(uuid.as_str(), 2, String::from("Job failed."),
                          String::from("Drive does not exist."), false, true, 0.0);
        return;
    }


    if (output_device != "special:any"){
        if (drive_uuid.is_empty()) {
            println!("Failed to get or create drive UUID.");
            update_job_status(uuid.as_str(), 2, String::from("Job failed."),
                              String::from("Failed to get or create drive UUID."), false, true, 0.0);
            return;
        }
        if (drive_uuid != output_device) {
            println!("Drive UUID does not match job output device: {} != {}", drive_uuid, output_device);
            update_job_status(uuid.as_str(), 2, String::from("Job failed."),
                              String::from("Drive UUID does not match job output device."), false, true, 0.0);
            return;
        }
    }

    // Ensure the output directory exists
    if !std::path::Path::new(&output_dir).exists() {
        if (job_info.new_folder.clone()) {
            match std::fs::create_dir_all(&output_dir) {
                Ok(_) => println!("Created output directory: {}", output_dir),
                Err(e) => {
                    println!("Failed to create output directory: {}", e);
                    update_job_status(uuid.as_str(), 2, String::from("Job failed."),
                                      String::from("Failed to create output directory."), false, true, 0.0);
                    return;
                }
            }
        }
        else {
            println!("Output directory does not exist and new_folder is false: {}", output_dir);
            update_job_status(uuid.as_str(), 2, String::from("Job failed."),
                              String::from("Output directory does not exist."), false, true, 0.0);
            return;
        }
    }

}
