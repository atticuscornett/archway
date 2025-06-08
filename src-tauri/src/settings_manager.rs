use serde::{Deserialize, Serialize};
use crate::storage_manager::{file_with_executable, read_json_file};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SettingsJSON {
    pub run_on_startup: Option<bool>,
    pub log_level: Option<String>
}

fn fill_default_settings(settings: &mut SettingsJSON)  {
    let default_settings = SettingsJSON {
        run_on_startup: Option::from(true),
        log_level: Option::from("low".to_string())
    };

    if settings.run_on_startup.is_none() {
        settings.run_on_startup = default_settings.run_on_startup;
    }
    if settings.log_level.is_none() {
        settings.log_level = default_settings.log_level;
    }
}

pub fn get_settings() -> SettingsJSON {
    let mut settings = read_json_file::<SettingsJSON>(file_with_executable("archway_settings.json")).unwrap_or_else(|_| {
        let mut default_settings = SettingsJSON {
            run_on_startup: None,
            log_level: None
        };
        fill_default_settings(&mut default_settings);
        println!("Settings file not found or invalid, using default settings: {:?}", default_settings);
        default_settings
    });

    println!("{:?}", settings);
    fill_default_settings(&mut settings);

    settings
}

pub fn set_settings(settings: &SettingsJSON) -> bool {
    let mut settings_to_save = settings.clone();
    fill_default_settings(&mut settings_to_save);

    match serde_json::to_string(&settings_to_save) {
        Ok(json_string) => {
            std::fs::write(file_with_executable("archway_settings.json"), json_string).is_ok()
        },
        Err(err) => {
            println!("Error serializing settings to JSON: {}", err);
            false
        }
    }
}