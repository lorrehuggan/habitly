use std::collections::HashMap;

use chrono::{Datelike, Duration, Local, Weekday};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Timeline {
    days: HashMap<String, Vec<String>>,
}

impl Timeline {
    fn new() -> Self {
        let mut days_map: HashMap<String, Vec<String>> = HashMap::new();

        const NUM_OF_WEEKS: i64 = 30;

        let days_of_week = [
            Weekday::Sun.to_string(),
            Weekday::Mon.to_string(),
            Weekday::Tue.to_string(),
            Weekday::Wed.to_string(),
            Weekday::Thu.to_string(),
            Weekday::Fri.to_string(),
            Weekday::Sat.to_string(),
        ];

        for day in days_of_week.clone() {
            days_map.insert(day.to_string(), Vec::new());
        }

        let now = Local::now();
        let start_date = now - Duration::weeks(NUM_OF_WEEKS);

        for week in 0..NUM_OF_WEEKS {
            for i in 0..7 {
                let current_date = start_date + Duration::days((i + 2) + (week * 7));

                if let Some(day_name) =
                    days_of_week.get(current_date.weekday().number_from_sunday() as usize - 1)
                {
                    days_map
                        .get_mut(day_name)
                        .unwrap()
                        .push(current_date.to_string());
                }
            }
        }

        Timeline { days: days_map }
    }

    fn get_day(&self, day: &str) -> Vec<String> {
        self.days.get(day).unwrap().clone()
    }
}

#[tauri::command]
pub fn init_timeline() -> Timeline {
    Timeline::new()
}
