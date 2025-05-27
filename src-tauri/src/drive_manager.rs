use std::fs;
use std::path::Path;
use crate::structs::DriveInfoFile;

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
    }
    else {
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
