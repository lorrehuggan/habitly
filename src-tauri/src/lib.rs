mod handlers;
mod timeline;
mod utils;

use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

use handlers::{commits, habits, settings};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{App, Manager};
use timeline::graph;

struct AppState {
    db: Pool<Sqlite>,
    settings_dir: PathBuf,
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = init_db(app).await;
                let mut settings_dir = app.path().app_local_data_dir().unwrap();
                settings_dir.push("settings.json");

                if !settings_dir.exists() {
                    println!("Settings file does not exist, creating new settings file");
                    let settings = settings::Settings::new();
                    let settings_json = serde_json::to_string(&settings);
                    match settings_json {
                        Ok(settings_json) => match fs::write(&settings_dir, settings_json) {
                            Ok(_) => {
                                println!("Settings file created at: {}", settings_dir.display())
                            }
                            // TODO: error handling
                            Err(e) => println!("Error writing settings file: {}", e),
                        },
                        Err(e) => println!("Error serializing settings: {}", e),
                    }
                }

                let state = AppState { db, settings_dir };

                app.manage(state);
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commits::get_habit_commits,
            commits::create_commit,
            commits::delete_commit,
            //--->
            graph::init_timeline,
            //--->
            habits::get_all_habits,
            habits::get_habit_by_id,
            habits::get_archived_habits,
            habits::create_habit,
            habits::update_habit,
            habits::delete_habit,
            habits::archive_habit,
            habits::restore_habit,
            //--->
            settings::get_settings,
            settings::update_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn init_db(app: &App) -> Pool<Sqlite> {
    let mut path = app
        .path()
        .app_local_data_dir()
        .expect("Error getting app data directory");

    path.push("habitly.sqlite");

    let result = OpenOptions::new().create_new(true).write(true).open(&path);

    match result {
        Ok(_) => println!("Database created at: {:?}", path),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => println!("Database already exists at: {:?}", path),
            _ => panic!("Error creating database: {:?}", e),
        },
    }

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .expect("Error connecting to database");

    println!("Migrating database");
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Error migrating database");

    db
}
