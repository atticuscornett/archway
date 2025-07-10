mod background_manager;
mod drive_manager;
mod job_manager;
mod log_manager;
mod settings_manager;
mod storage_manager;
mod structs;

use serde_json;
use std::collections::HashMap;
use std::hash::Hash;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::drive_manager::get_root_drive;
use crate::job_manager::get_app_handle;
use crate::settings_manager::SettingsJSON;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use sysinfo::Disks;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_autostart::ManagerExt;

static APP_HANDLE: OnceCell<Mutex<AppHandle>> = OnceCell::new();

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
fn clear_completed_jobs() {
    job_manager::clear_completed_jobs();
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

#[tauri::command]
fn pause_job(uuid: String) {
    job_manager::set_job_update(uuid.clone(), "pause_requested".to_string());
}

#[tauri::command]
fn unpause_job(uuid: String) {
    job_manager::set_job_update(uuid.clone(), "running".to_string());
}

#[tauri::command]
fn get_job_update(uuid: String) -> String {
    job_manager::get_job_update(uuid.clone())
}

#[tauri::command]
fn stop_job(uuid: String) {
    job_manager::set_job_update(uuid.clone(), "stop_requested".to_string());
}

#[tauri::command]
fn get_all_job_health() -> HashMap<String, String> {
    storage_manager::get_all_job_health()
}

#[tauri::command]
fn get_settings() -> SettingsJSON {
    settings_manager::get_settings()
}

#[tauri::command]
fn set_settings(settings: SettingsJSON) -> bool {
    println!("Setting settings: {:?}", settings);
    settings_manager::set_settings(&settings)
}

#[tauri::command]
fn get_job_file_type(file: String) -> String {
    storage_manager::get_job_file_type(&file)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cws| {
            let main_window = app.get_webview_window("main").expect("no main window");
            main_window.show().unwrap();
            main_window.set_focus().unwrap();
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_drives,
            get_documents,
            setup_job,
            get_all_jobs,
            get_job_by_uuid,
            remove_job_by_uuid,
            get_all_job_statuses,
            start_job,
            clear_completed_jobs,
            pause_job,
            unpause_job,
            stop_job,
            get_job_update,
            get_all_job_health,
            get_settings,
            set_settings,
            get_job_file_type
        ])
        .setup(|app| {
            // Store the app handle in a global variable for later use
            APP_HANDLE.set(Mutex::new(app.handle().clone())).unwrap();
            job_manager::set_app_handle(app.handle().clone());

            let autostart_manager = app.autolaunch();
            let settings = settings_manager::get_settings();
            if settings.run_on_startup.unwrap_or(true) {
                autostart_manager.enable().unwrap_or_else(|err| {
                    println!("Failed to enable autostart: {}", err);
                });
            } else {
                autostart_manager.disable().unwrap_or_else(|err| {
                    println!("Failed to disable autostart: {}", err);
                });
            }


            tauri::async_runtime::spawn(background_manager::background_worker());

            let app_title = MenuItem::with_id(app, "app_title", "Archway", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&app_title, &quit_i])?;

            // Tray icon setup
            let mut tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true);

            tray = tray.on_menu_event(|app, event| {
                if event.id() == "quit" {
                    let running_jobs = job_manager::get_all_job_statuses();
                    let mut job_count = 99;
                    let mut iter = 0;

                    // Hide main window
                    let main_window = app.get_webview_window("main").expect("no main window");
                    main_window.hide().unwrap();

                    // Wait for jobs to stop or stop after 30 seconds
                    while iter < 3 && job_count > 0{
                        job_count = 0;
                        for job in running_jobs.clone() {
                            if job.completed || !job.success {
                                continue;
                            }
                            job_count += 1;
                            job_manager::set_job_update(job.job.uuid, "stop_requested".to_string());
                        }

                        if job_count > 0{
                            // Give jobs time to stop
                            std::thread::sleep(std::time::Duration::from_secs(10));

                            if iter == 0 {
                                // Notify user Archway is stopping jobs
                                get_app_handle()
                                    .notification()
                                    .builder()
                                    .title("Archway is stopping jobs...")
                                    .body("Archway is attempting to stop all running jobs. This may take a few moments.")
                                    .show()
                                    .unwrap();
                            }
                        }

                        iter += 1;
                    }




                    app.exit(0);
                } else if event.id() == "app_title" {
                    // Open Main Window
                    println!("Opening main window");
                    let main_window = app.get_webview_window("main").expect("no main window");
                    main_window.show().unwrap();
                    main_window.set_focus().unwrap();
                }
            });

            tray.build(app)?;

            // Hide main window on startup
            let main_window = app.get_webview_window("main").expect("no main window");
            if (settings.run_on_startup.unwrap_or(true)) {
                main_window.hide().unwrap();
            }
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
