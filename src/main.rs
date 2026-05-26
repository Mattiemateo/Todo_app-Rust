use std::env;
use std::fs;

#[derive(serde::Serialize, serde::Deserialize)]
struct Todo {
    id: usize,
    text: String,
    done: bool,
}
const TODOS_FILE: &str = "todos.json";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <add|list|done|delete> [args]");
        return;
    }

    let arg = args[1].clone();

    match arg.as_str() { 
        "add" => add_todo(args),
        "list" => list_todos(),
        "done" => mark_done(args),
        "delete" => delete_todo(args),
        "purge" => purge_todos(args),
        _ => println!("Unknown command: {}", args[0]),
    }
}

fn add_todo(args: Vec<String>) {
    if args.len() < 3 {
        println!("Usage: todo add <task description>");
        return;
    }
    let task = args[2..].join(" ");
    
    let mut todos = load_todos();
    let new_id = todos.len() + 1;

    todos.push(Todo {
        id: new_id,
        text: task.clone(),
        done: false,
    });
    
    save_todos(&todos);
    println!("Added todo: {}", task);
}
fn list_todos(){
    let todos = load_todos();
    if todos.len() == 0 {
        println!("No todos found, relax! :)");
    }
    for todo in todos {
        println!("[{}] {}: {}", if todo.done { "x" } else { " " }, todo.id, todo.text);
    }
}

fn mark_done(args: Vec<String>){
    if args.len() < 3 {
        println!("Usage: todo done <todo id>");
        return;
    }
    let id = args[2]
        .parse::<usize>()
        .expect("Please provide a valid todo id");
    let mut todos = load_todos();
    for todo in &mut todos {
        if todo.id == id {
            todo.done = true;
            save_todos(&todos);
            println!("Marked todo {} as done", id);
            return;
        }
    }
}

fn delete_todo(args: Vec<String>){
    if args.len() < 3 {
        println!("Usage: todo delete <todo id>");
        return;
    }
    let id = args[2]
        .parse::<usize>()
        .expect("Please provide a valid todo id");
    let mut todos = load_todos();
    if let Some(pos) = todos.iter().position(|todo| todo.id == id) {
        todos.remove(pos);
        save_todos(&todos);
        println!("Deleted todo {}", id);
    }
}

fn save_todos(todos: &[Todo]) {
    let json = serde_json::to_string_pretty(todos).unwrap();
    fs::write(TODOS_FILE, json).unwrap();
}

fn load_todos() -> Vec<Todo> {
    match fs::read_to_string(TODOS_FILE) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

fn purge_todos(args: Vec<String>) {
    if args.len() < 2 {
        println!("Usage: todo purge");
        return;
    }

    let mut todos = load_todos();
    todos.retain(|todo| !todo.done);
    save_todos(&todos);
    println!("Purged completed todos");
}
