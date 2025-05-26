mod structs;

use serde_json;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use sysinfo::Disks;

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

#[tauri::command]
fn setup_job(job_info: String) -> bool {
    let new_job: structs::JobInfo = match get_job_from_string(&job_info) {
        Ok(job) => job,
        Err(err) => {
            println!("{}", err);
            return false;
        },
    };

    println!("{}", new_job.job_name);
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
