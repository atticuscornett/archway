use tauri_plugin_notification::NotificationExt;
use crate::drive_manager::get_drive_uuid;
use crate::structs::JobInfo;
use crate::{drive_manager, job_manager, storage_manager, structs};
use time::OffsetDateTime;

pub async fn background_worker() {
    let mut drives = drive_manager::get_all_drives();

    loop {
        let updated_drives = drive_manager::get_all_drives();
        // Handle drive connection job triggers
        if updated_drives != drives {
            println!("Drive list has changed, updating...");

            let mut new_drives: Vec<Vec<String>> = Vec::new();

            for drive in &updated_drives {
                if !drives.contains(drive) {
                    new_drives.push(drive.clone());
                    println!("New drive detected: {:?}", drive);
                }
            }

            drives = updated_drives;

            let all_jobs = storage_manager::get_all_jobs();
            let mut jobs_with_drive_trigger: Vec<JobInfo> = Vec::new();

            for job in all_jobs {
                for trigger in &job.triggers {
                    if trigger.trigger_type == "event"
                        && trigger.clone().traits.event.unwrap() == "device-connection"
                    {
                        jobs_with_drive_trigger.push(job.clone());
                    }
                }
            }

            for job in jobs_with_drive_trigger {
                if job.output_device == "special:any" {
                    let root_drive = match drive_manager::get_root_drive(&job.output_dir) {
                        Some(drive) => drive,
                        None => {
                            println!("Failed to determine root drive for output directory.");
                            continue;
                        }
                    };

                    println!("Root drive found: {}", root_drive);

                    for new_drive in &new_drives {
                        if new_drive.get(0).unwrap().starts_with(&root_drive) {
                            println!(
                                "Triggering job {} for new drive {}",
                                job.clone().job_name,
                                root_drive
                            );
                            job_manager::start_job(job.clone().uuid);
                        }
                    }
                } else {
                    let required_drive = job.output_device.clone();

                    for new_drive in &new_drives {
                        let drive_uuid = get_drive_uuid(new_drive.get(0).unwrap());

                        if drive_uuid == required_drive {
                            println!(
                                "Triggering job {} for new drive {}",
                                job.clone().job_name,
                                required_drive
                            );
                            job_manager::start_job(job.clone().uuid);
                        }
                    }
                }
            }

            // Check if jobs are available for import from new drives
            for new_drive in &new_drives {
                // Check if the drive contains an archway.json file
                let drive_uuid = get_drive_uuid(new_drive.get(0).unwrap());
                if drive_uuid.is_empty() {
                    println!("No UUID found for new drive: {:?}", new_drive);
                    continue;
                }

                println!("Checking drive: {} with UUID: {} for new jobs", new_drive.get(0).unwrap(), drive_uuid);
                let drive_path_string = new_drive.get(0).unwrap();
                let drive_path = std::path::Path::new(drive_path_string);
                let drive_info_path = drive_path.join("archway.json");
                let drive_jobs = storage_manager::read_json_file::<structs::DriveInfoFile>(drive_info_path.to_string_lossy().to_string());

                println!("Drive info path: {}", drive_info_path.display());
                match drive_jobs {
                    Ok(drive_info) => {
                        let mut jobs_available = false;
                        for job in drive_info.jobs {
                            println!("Checking job: {} with UUID: {}", job.job_name, job.uuid);
                            if !storage_manager::get_all_jobs().iter().any(|j| j.uuid == job.uuid) {
                                println!("Job {} is available for import", job.job_name);
                                jobs_available = true;
                            }
                        }

                        if jobs_available {
                            println!("New jobs available for import from drive: {}", drive_uuid);
                            let result = job_manager::get_app_handle()
                                .notification()
                                .builder()
                                .title("New Jobs Available")
                                .body("New jobs are available for import from a connected drive. See the job creation page for more details.")
                                .show();

                            match result {
                                Ok(_) => println!("Notification sent successfully."),
                                Err(e) => println!("Failed to send notification: {}", e),
                            }
                        }
                    },
                    Err(e) => {
                        println!("Failed to read drive info file: {}", e);
                    }
                }
            }

        } else {
            println!("No changes in drive list.");
        }

        // Handle periodic job triggers
        let all_jobs = storage_manager::get_all_jobs();
        let times_index = vec![
            "12 AM", "1 AM", "2 AM", "3 AM", "4 AM", "5 AM", "6 AM", "7 AM", "8 AM", "9 AM",
            "10 AM", "11 AM", "12 PM", "1 PM", "2 PM", "3 PM", "4 PM", "5 PM", "6 PM", "7 PM",
            "8 PM", "9 PM", "10 PM", "11 PM",
        ];
        let weekday_index = vec![
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        for job in all_jobs {
            for trigger in &job.triggers {
                if trigger.clone().trigger_type == "time" {
                    let current_time = OffsetDateTime::now_local().unwrap();
                    let current_hour = current_time.hour();
                    let current_minute = current_time.minute();
                    let current_day = current_time.day();
                    let current_weekday = current_time.weekday();
                    println!(
                        "Current time: {}:{} on day {}, weekday {}",
                        current_hour, current_minute, current_day, current_weekday
                    );
                    if trigger.clone().traits.event.unwrap() == "hourly" {
                        if current_minute == 0 {
                            println!("Triggering hourly job: {}", job.clone().job_name);
                            job_manager::start_job(job.clone().uuid);
                        }
                    }

                    if trigger.clone().traits.event.unwrap() == "daily" {
                        let trigger_times = trigger.clone().traits.time.unwrap();
                        let hour_trigger = trigger_times.get(0);
                        if let Some(hour_str) = hour_trigger {
                            if hour_str == &times_index[current_hour as usize]
                                && current_minute == 0
                            {
                                println!("Triggering daily job: {}", job.clone().job_name);
                                job_manager::start_job(job.clone().uuid);
                            }
                        }
                    }

                    if trigger.clone().traits.event.unwrap() == "weekly" {
                        let trigger_times = trigger.clone().traits.time.unwrap();
                        let weekday_trigger = trigger_times.get(0).unwrap();
                        let time_trigger = trigger_times.get(1);

                        println!(
                            "Current weekday: {}, Trigger weekday: {:?}",
                            current_weekday, &weekday_index[current_weekday as usize]
                        );

                        if weekday_trigger == &weekday_index[current_weekday as usize] {
                            if let Some(hour_str) = time_trigger {
                                if hour_str == &times_index[current_hour as usize]
                                    && current_minute == 0
                                {
                                    println!("Triggering weekly job: {}", job.clone().job_name);
                                    job_manager::start_job(job.clone().uuid);
                                }
                            }
                        }
                    }

                    if trigger.clone().traits.event.unwrap() == "monthly" {
                        let trigger_times = trigger.clone().traits.time.unwrap();
                        let day_trigger = trigger_times.get(0).unwrap();
                        let time_trigger = trigger_times.get(1);

                        println!(
                            "Current weekday: {}, Trigger weekday: {:?}",
                            current_weekday, &weekday_index[current_weekday as usize]
                        );

                        if day_trigger.parse::<u8>().unwrap() == current_day {
                            if let Some(hour_str) = time_trigger {
                                if hour_str == &times_index[current_hour as usize]
                                    && current_minute == 0
                                {
                                    println!("Triggering monthly job: {}", job.clone().job_name);
                                    job_manager::start_job(job.clone().uuid);
                                }
                            }
                        }
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
