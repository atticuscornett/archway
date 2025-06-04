use crate::drive_manager;

pub async fn background_worker() {
    let mut drives = drive_manager::get_all_drives();

    while true {
        let mut updated_drives = drive_manager::get_all_drives();
        if updated_drives != drives {
            println!("Drive list has changed, updating...");
            drives = updated_drives;
        } else {
            println!("No changes in drive list.");
        }


        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}