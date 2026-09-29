mod models;
mod storage;

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let mut data = storage::load()?;

    if let Some(command) = args.get(1) {
        match command.as_str() {
            "add" => {
                if let (Some(header), Some(summary)) = (args.get(2), args.get(3)) {
                    data.add_task(header.clone(), summary.clone());
                    storage::save(&data)?;
                    println!("Task added.");
                } else {
                    return Err("Usage: todo add <header> <summary>".to_string());
                }
            }
            "list" => {
                if data.tasks.is_empty() {
                    println!("No tasks yet.");
                } else {
                    data.list_task();
                }
            }
            "complete" => {
                if let Some(id_str) = args.get(2) {
                    if let Ok(id) = id_str.parse::<u32>() {
                        if data.complete_task(id) {
                            storage::save(&data)?;
                            println!("Task {id} marked as done.");
                        } else {
                            return Err(format!("no task found with id {id}."));
                        }
                    } else {
                        return Err(format!("'{id_str}' is not a valid task id."));
                    }
                } else {
                    return Err("Usage: todo complete <id>".to_string());
                }
            }
            "remove" => {
                if let Some(id) = args.get(2)
                    && let Ok(id) = id.parse::<u32>()
                {
                    if data.remove_task(id) {
                        storage::save(&data)?;
                        println!("Task {id} was removed.");
                    } else {
                        return Err(format!("no task found with id {id}."));
                    }
                } else {
                    return Err("Usage: todo remove <id>".to_string());
                }
            }
            _ => {
                return Err("Usage: todo <add|list|complete|remove> [options]".to_string());
            }
        }
    } else {
        return Err("Usage: todo <add|list|complete|remove> [options]".to_string());
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
