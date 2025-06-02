mod drive_manager;
mod structs;
mod storage_manager;
mod job_manager;

use serde_json;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::Path;
use sysinfo::Disks;
use tauri::{command, AppHandle};
use crate::drive_manager::get_root_drive;

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

#[tauri::command]
fn get_all_jobs() -> String {
    let all_jobs = storage_manager::get_all_jobs();
    serde_json::to_string(&all_jobs).unwrap_or_else(|err| {
        println!("Error serializing jobs to JSON: {}", err);
        String::new()
    })
}

#[tauri::command]
fn get_job_by_uuid(uuid: String) -> String {
    let job = storage_manager::get_job_by_uuid(&uuid);
    serde_json::to_string(&job).unwrap_or_else(|err| {
        println!("Error serializing job to JSON: {}", err);
        String::new()
    })
}

#[tauri::command]
fn remove_job_by_uuid(uuid: String) -> bool {
    if storage_manager::remove_job_by_uuid(&uuid) {
        println!("Job with UUID {} removed successfully.", uuid);
        return true;
    } else {
        println!("Failed to remove job with UUID {}.", uuid);
        return false;
    }
}

fn get_job_from_string(job_info: &str) -> Result<structs::JobInfo, serde_json::Error> {
    serde_json::from_str(job_info)
}

#[tauri::command]
fn start_job(uuid: String) -> bool {
    if job_manager::start_job(uuid) {
        println!("Job started successfully.");
        return true;
    } else {
        println!("Failed to start job.");
        return false;
    }
}

#[tauri::command]
fn get_all_job_statuses() -> String {
    let statuses = job_manager::get_all_job_statuses();
    serde_json::to_string(&statuses).unwrap_or_else(|err| {
        println!("Error serializing job statuses to JSON: {}", err);
        String::new()
    })
}

#[tauri::command]
fn setup_job(job_info: String) -> bool {
    let mut new_job: structs::JobInfo = match get_job_from_string(&job_info) {
        Ok(job) => job,
        Err(err) => {
            println!("{}", err);
            return false;
        }
    };

    // Assign job to drive
    if new_job.output_device == "special:thisdrive" {
        // Determine drive from output folder
        let root_drive = match get_root_drive(&new_job.output_dir) {
            Some(drive) => drive,
            None => {
                println!("Failed to determine root drive for output directory.");
                return false;
            }
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

    if new_job.portable {
        let root_drive = match get_root_drive(&new_job.output_dir) {
            Some(drive) => drive,
            None => {
                println!("Failed to determine root drive for output directory.");
                return false;
            }
        };
        drive_manager::add_job_to_drive(&root_drive, new_job.clone());
    }

    let job_json = match serde_json::to_string(&new_job) {
        Ok(json) => json,
        Err(err) => {
            println!("{}", err);
            return false;
        }
    };

    let mut all_jobs = storage_manager::get_all_jobs();
    let job_uuid = new_job.uuid.clone();
    // Check if job with the same UUID already exists
    if let Some(existing_job) = all_jobs.iter_mut().find(|job| job.uuid == job_uuid) {
        // Update existing job
        *existing_job = new_job.clone();
    } else {
        // Add new job
        all_jobs.push(new_job.clone());
    }
    
    if !storage_manager::set_all_jobs(all_jobs) {
        println!("Failed to save jobs to storage.");
        return false;
    }

    println!("{}", new_job.job_name);
    println!("{}", job_json);
    return true;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_drives, get_documents, setup_job,
            get_all_jobs, get_job_by_uuid, remove_job_by_uuid, get_all_job_statuses, start_job])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
