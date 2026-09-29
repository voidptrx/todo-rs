use crate::models::TodoData;
use std::io::ErrorKind;

const DATA_FILE: &str = "tasks.json";

pub fn load() -> Result<TodoData, String> {
    load_task(DATA_FILE)
}

pub fn save(data: &TodoData) -> Result<(), String> {
    save_task(data, DATA_FILE)
}

fn load_task(path: &str) -> Result<TodoData, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(TodoData::default()),
        Err(e) => return Err(format!("could not read task file '{path}': '{e}'")),
    };

    serde_json::from_str::<TodoData>(&content)
        .map_err(|e| format!("task file '{path}' is corrupted: {e}"))
}

fn save_task(data: &TodoData, path: &str) -> Result<(), String> {
    let json =
        serde_json::to_string_pretty(data).map_err(|e| format!("JSON creation error: {e}"))?;

    std::fs::write(path, json).map_err(|e| format!("Error during writing to file: {e}"))
}
