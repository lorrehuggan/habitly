use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{utils::TauriResponse, AppState};

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
            status: String::from("ongoing"),
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
        .filter(|habit| habit.status != "archived")
        .collect();

    Ok(habits)
}
#[tauri::command]
pub async fn get_habit_by_id(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Habit, String> {
    let db = &state.db;

    let habit: Habit = sqlx::query_as("SELECT * FROM habits WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get habit by id {}", e))?;
    Ok(habit)
}

#[tauri::command]
pub async fn create_habit(
    state: tauri::State<'_, AppState>,
    habit: Habit,
) -> Result<TauriResponse, String> {
    let db = &state.db;

    let habit = Habit::new(
        habit.title,
        habit.description,
        habit.color,
        habit.streak,
        habit.category,
    );

    let query = "INSERT INTO habits (id, name, description, created, color, streak, category, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

    sqlx::query(query)
        .bind(habit.id)
        .bind(habit.title)
        .bind(habit.description)
        .bind(habit.created)
        .bind(habit.color)
        .bind(habit.streak)
        .bind(habit.category)
        .bind(habit.status)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to create habit {}", e))?;

    Ok(TauriResponse {
        message: "success".to_string(),
        error: false,
    })
}

#[tauri::command]
pub async fn archive_habit(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<TauriResponse, String> {
    let db = &state.db;

    let query = "UPDATE habits SET status = 'archived' WHERE id = $1";

    sqlx::query(query)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to archive habit {}", e))?;

    Ok(TauriResponse {
        message: "success".to_string(),
        error: false,
    })
}

#[tauri::command]
pub async fn update_habit(
    state: tauri::State<'_, AppState>,
    habit: Habit,
) -> Result<TauriResponse, String> {
    let db = &state.db;

    let query = "UPDATE habits SET name = $1, description = $2, color = $3, streak = $4, category = $5 WHERE id = $6";

    sqlx::query(query)
        .bind(habit.title)
        .bind(habit.description)
        .bind(habit.color)
        .bind(habit.streak)
        .bind(habit.category)
        .bind(habit.id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update habit {}", e))?;

    Ok(TauriResponse {
        message: "success".to_string(),
        error: false,
    })
}

#[tauri::command]
pub async fn get_archived_habits(state: tauri::State<'_, AppState>) -> Result<Vec<Habit>, String> {
    let db = &state.db;

    let habits: Vec<Habit> = sqlx::query_as("SELECT * FROM habits")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get todos {}", e))?;

    let habits = habits
        .into_iter()
        .filter(|habit| habit.status == "archived")
        .collect();

    Ok(habits)
}

#[tauri::command]
pub async fn delete_habit(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<TauriResponse, String> {
    let db = &state.db;

    let query1 = "DELETE FROM commits WHERE habit_id = $1";
    let query2 = "DELETE FROM habits WHERE id = $1";

    //TODO: Add transaction

    sqlx::query(query1)
        .bind(&id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete commits {}", e))?;

    sqlx::query(query2)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete habit {}", e))?;

    Ok(TauriResponse {
        message: "success".to_string(),
        error: false,
    })
}

#[tauri::command]
pub async fn restore_habit(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<TauriResponse, String> {
    let db = &state.db;

    let query = "UPDATE habits SET status = 'ongoing' WHERE id = $1";

    sqlx::query(query)
        .bind(id)
        .execute(db)
        .await
        .expect("Failed to restore habit");

    Ok(TauriResponse {
        message: "success".to_string(),
        error: false,
    })
}
