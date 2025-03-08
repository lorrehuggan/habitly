use chrono::{Datelike, Duration, Local, Weekday};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Timeline {
    days: HashMap<String, Vec<String>>,
}

impl Timeline {
    fn default() -> Self {
        let mut days_map: HashMap<String, Vec<String>> = HashMap::new();

        let days_of_week = vec![
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

        let today = Local::now();

        for i in 0..52 {
            let mut current_week_start = today - Duration::weeks(i);

            while current_week_start.weekday() != Weekday::Sun {
                current_week_start -= Duration::days(1);
            }

            for j in 0..7 {
                let current_date = current_week_start + Duration::days(j);

                if let Some(day_name) =
                    days_of_week.get(current_date.weekday().number_from_sunday() as usize - 1)
                {
                    match days_map.get_mut(day_name) {
                        Some(map) => map.push(current_date.format("%Y-%m-%d").to_string()),
                        None => println!("Day not found: {}", day_name),
                    }
                }
            }
        }

        Timeline { days: days_map }
    }
}

#[tauri::command]
pub fn init_timeline() -> Timeline {
    Timeline::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::default();
        assert!(timeline.days.len() == 7, "Timeline should have 7 days");

        for day in timeline.days.keys() {
            assert!(
                timeline.days.get(day).unwrap().len() == 52,
                "Timeline should have 52 dates per day"
            );
        }
    }
    #[test]
    fn test_timeline_contains_all_days() {
        let timeline = Timeline::default();
        let expected_days = [
            Weekday::Sun.to_string(),
            Weekday::Mon.to_string(),
            Weekday::Tue.to_string(),
            Weekday::Wed.to_string(),
            Weekday::Thu.to_string(),
            Weekday::Fri.to_string(),
            Weekday::Sat.to_string(),
        ];
        for day in expected_days {
            assert!(
                timeline.days.contains_key(&day),
                "Timeline should contain the day: {}",
                day
            );
        }
    }

    #[test]
    fn test_timeline_date_range() {
        let timeline = Timeline::default();
        let today = Local::now();

        for i in 0..52 {
            let mut current_week_start = today - chrono::Duration::weeks(i as i64);

            while current_week_start.weekday() != Weekday::Sun {
                current_week_start -= chrono::Duration::days(1);
            }

            for j in 0..7 {
                let expected_date = current_week_start + chrono::Duration::days(j as i64);
                if let Some(day_name) = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
                    .get(expected_date.weekday().number_from_sunday() as usize - 1)
                {
                    let actual_dates: Vec<&str> = timeline
                        .days
                        .get(day_name.to_owned())
                        .unwrap()
                        .iter()
                        .map(|d| d.as_str())
                        .collect();

                    assert!(
                        actual_dates
                            .contains(&expected_date.format("%Y-%m-%d").to_string().as_str()),
                        "Date {} should be in the list for {}",
                        expected_date.format("%Y-%m-%d"),
                        day_name
                    );
                }
            }
        }
    }
}
