use crate::Task;
use std::fs;
use std::io::ErrorKind;

pub fn load_tasks(path: &str) -> Result<Vec<Task>, String> {
    match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str::<Vec<Task>>(&contents).map_err(|e| e.to_string()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn save_tasks(path: &str, tasks: &[Task]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(tasks).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
