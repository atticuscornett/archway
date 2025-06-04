use crate::structs::{DriveInfoFile, JobInfo};
use std::fs;
use std::path::Path;
use sysinfo::Disks;

pub fn get_drive_uuid(drive_name: &str) -> String {
    let uuid_drive = Path::new(drive_name);
    let uuid_path = uuid_drive.join("archway.json");
    let mut uuid_string: String = String::new();

    if uuid_path.exists() {
        let drive_info: DriveInfoFile;
        let file = fs::File::open(&uuid_path);
        drive_info = match {
            let file = file.unwrap();
            serde_json::from_reader(file)
        } {
            Ok(info) => info,
            Err(e) => {
                println!("Failed to read drive info file: {}", e);
                return uuid_string;
            }
        };

        uuid_string = drive_info.uuid.clone();
    } else {
        let uuid = uuid::Uuid::new_v4();
        uuid_string = uuid.to_string();
        let new_drive_info = serde_json::json!({
            "uuid": uuid_string,
            "jobs": []
        });
        match std::fs::write(&uuid_path, new_drive_info.to_string()) {
            Ok(_) => println!("Created new drive info file at {}", uuid_path.display()),
            Err(e) => {
                println!("Failed to create drive info file: {}", e);
                return uuid_string;
            }
        }
    }

    return uuid_string;
}

pub fn add_job_to_drive(drive_name: &str, job_info: JobInfo) -> bool {
    let uuid_drive = Path::new(drive_name);
    let uuid_path = uuid_drive.join("archway.json");

    if !uuid_path.exists() {
        println!("Drive info file does not exist at {}", uuid_path.display());
        let drive_uuid = get_drive_uuid(drive_name);

        if drive_uuid.is_empty() {
            println!("Failed to get or create drive UUID.");
            return false;
        }
    }

    let mut drive_info: DriveInfoFile;
    let file = fs::File::open(&uuid_path);
    match file {
        Ok(file) => {
            drive_info = serde_json::from_reader(file).unwrap();
        }
        Err(e) => {
            println!("Failed to open drive info file: {}", e);
            return false;
        }
    }

    let mut already_exists = false;

    // Check if job already exists, if so, update it
    for existing_job in &mut drive_info.jobs {
        if existing_job.uuid == job_info.uuid {
            *existing_job = job_info.clone();
            already_exists = true;
        }
    }

    // If job does not exist, add it
    if !already_exists {
        drive_info.jobs.push(job_info.clone());
    }

    match std::fs::write(&uuid_path, serde_json::to_string(&drive_info).unwrap()) {
        Ok(_) => true,
        Err(e) => {
            println!("Failed to write updated drive info file: {}", e);
            false
        }
    }
}

pub fn get_root_drive(path: &str) -> Option<String> {
    let path = Path::new(path);
    if let Some(component) = path.components().next() {
        if let std::path::Component::Prefix(prefix_component) = component {
            return Some(prefix_component.as_os_str().to_string_lossy().to_string());
        }
    }
    None
}

pub fn get_all_drives() -> Vec<Vec<String>> {
    let disks = Disks::new_with_refreshed_list();
    let mut drive_list: Vec<Vec<String>> = Vec::new();

    for disk in disks.list() {
        drive_list.push(vec![
            disk.mount_point().to_string_lossy().to_string(),
            disk.name().to_string_lossy().to_string(),
        ]);
    }

    return drive_list
}