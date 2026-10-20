use std::{ env, fs::File, io::{ Read, Write } };
use console::Style;

const FILE_NAME: &str = "./todos.txt";
mod todo;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let strike = Style::new().strikethrough();

    let todos = get_todos();

    if arguments.len() > 1 {
        let command = arguments[1].as_str();
        let args = &arguments[2..];

        match command {
            "add" => {
                if args.len() > 0 {
                    for arg in args {
                        add_todo_to_file(arg.to_string());
                    }
                }
            }
            "done" => {}
            _ => {}
        }
    } else {
        for todo in todos {
            if todo.is_complete {
                println!("Task: {}", strike.apply_to(todo.task));
            } else {
                println!("Task: {}", todo.task);
            }
        }
    }
}

fn add_todo_to_file(todo_content: String) {
    let mut new_todo = String::from("[] ");
    new_todo.push_str(todo_content.as_str());
    (
        match File::options().append(true).open(FILE_NAME) {
            Ok(mut file) => writeln!(file, "{}", new_todo),
            Err(err) => panic!("{err}"),
        }
    ).unwrap();
}

fn update_todo_is_complete(id: u32) {}
fn delete_todo(id: u32) {}

fn get_file_content() -> Vec<String> {
    let mut file = match File::open(FILE_NAME) {
        Ok(file) => file,
        Err(err) => panic!("{err}"),
    };
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn convert_file_content_to_task(lines: Vec<String>) -> Vec<todo::Todo> {
    let mut todos: Vec<todo::Todo> = Vec::new();

    for line in lines {
        let tokens: Vec<String> = line
            .split_whitespace()
            .map(|token| token.to_string())
            .collect();

        let new_todo = todo::Todo::new(tokens[1].to_string(), tokens[0].contains("x"));
        todos.push(new_todo);
    }

    todos
}

fn get_todos() -> Vec<todo::Todo> {
    let todos: Vec<todo::Todo> = convert_file_content_to_task(get_file_content());

    return todos;
}
