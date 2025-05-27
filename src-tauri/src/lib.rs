mod structs;
mod drive_manager;

use serde_json;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use sysinfo::Disks;
use std::path::Path;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_drives() -> Vec<Vec<String>> {
    let disks = Disks::new_with_refreshed_list();

    let mut drives = Vec::new();

    for disk in disks.list() {
        let mut drive_info = Vec::new();
        drive_info.push(disk.mount_point().to_string_lossy().to_string());
        drive_info.push(disk.name().to_string_lossy().to_string());
        drives.push(drive_info);
    }

    return drives;
}

#[tauri::command]
fn get_documents() -> String {
    return dirs::document_dir().unwrap().to_string_lossy().to_string();
}

fn get_job_from_string(job_info: &str) -> Result<structs::JobInfo, serde_json::Error> {
    serde_json::from_str(job_info)
}

fn get_root_drive(path: &str) -> Option<String> {
    let path = Path::new(path);
    if let Some(component) = path.components().next() {
        if let std::path::Component::Prefix(prefix_component) = component {
            return Some(prefix_component.as_os_str().to_string_lossy().to_string());
        }
    }
    None
}


#[tauri::command]
fn setup_job(job_info: String) -> bool {
    let mut new_job: structs::JobInfo = match get_job_from_string(&job_info) {
        Ok(job) => job,
        Err(err) => {
            println!("{}", err);
            return false;
        },
    };

    // Assign job to drive
    if new_job.output_device == "special:thisdrive" {
        // Determine drive from output folder
        let root_drive = match get_root_drive(&new_job.output_dir) {
            Some(drive) => drive,
            None => {
                println!("Failed to determine root drive for output directory.");
                return false;
            },
        };
        println!("Root drive is {}", root_drive);

        // Determine or create drive UUID
        let drive_uuid = drive_manager::get_drive_uuid(&root_drive);
        if drive_uuid.is_empty() {
            println!("Failed to get or create drive UUID.");
            return false;
        }
        println!("Drive UUID is {}", drive_uuid);

        // Update job with drive UUID
        new_job.output_device = drive_uuid.clone();
    }

    let job_json = match serde_json::to_string(&new_job) {
        Ok(json) => json,
        Err(err) => {
            println!("{}", err);
            return false;
        },
    };

    println!("{}", new_job.job_name);
    println!("{}", job_json);
    return true;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_drives])
        .invoke_handler(tauri::generate_handler![get_documents])
        .invoke_handler(tauri::generate_handler![setup_job])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
