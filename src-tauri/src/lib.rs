mod handlers;
mod timeline;

use std::fs::OpenOptions;

use handlers::habits;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{App, Manager};
use timeline::graph;

struct AppState {
    db: Pool<Sqlite>,
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = init_db(app).await;
                let state = AppState { db };

                app.manage(state);
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            graph::init_timeline,
            habits::get_all_habits
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
    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    db
}
