use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Habit {
    id: String,
    title: String,
    description: String,
    created: String,
    color: String,
    streak: u8,
    category: String,
    status: String,
}

impl Habit {
    pub fn new(
        title: String,
        description: String,
        color: String,
        streak: u8,
        category: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            created: chrono::Local::now().to_string(),
            color,
            streak,
            category,
            status: String::from("Active"),
        }
    }
}

#[tauri::command]
pub async fn get_all_habits(state: tauri::State<'_, AppState>) -> Result<Vec<Habit>, String> {
    let db = &state.db;

    let habits: Vec<Habit> = sqlx::query_as("SELECT * FROM habits")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get habits {}", e))?;

    let habits = habits
        .into_iter()
        .filter(|habit| habit.status != "Archived")
        .collect();

    Ok(habits)
}
