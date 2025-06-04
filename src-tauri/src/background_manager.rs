use crate::{drive_manager, job_manager, storage_manager};
use crate::job_manager::start_job;
use crate::structs::JobInfo;

pub async fn background_worker() {
    let mut drives = drive_manager::get_all_drives();

    loop {
        let updated_drives = drive_manager::get_all_drives();
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
                    if trigger.trigger_type == "event" && trigger.clone().traits.event.unwrap() == "device-connection" {
                        jobs_with_drive_trigger.push(job.clone());
                    }
                }
            }

            for job in jobs_with_drive_trigger {
                if (job.output_device == "special:any"){
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
                            println!("Triggering job {} for new drive {}", job.clone().job_name, root_drive);
                            job_manager::start_job(job.clone().uuid);
                        }
                    }
                }
            }

        } else {
            println!("No changes in drive list.");
        }


        std::thread::sleep(std::time::Duration::from_secs(15));
    }
}