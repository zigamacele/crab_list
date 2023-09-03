use std::io;

struct TodoApp {
    todo_list: Vec<String>,
}

enum Action {
    Add,
    Edit,
    Remove,
    View,
    Exit,
    Invalid,
}

impl TodoApp {
    fn new() -> TodoApp {
        TodoApp {
            todo_list: Vec::new(),
        }
    }

    fn read_input(&mut self) -> Action {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "0" => Action::Add,
            "1" => Action::Edit,
            "2" => Action::Remove,
            "3" => Action::View,
            "4" => Action::Exit,
            _ => Action::Invalid,
        }
    }

    fn add_todo(&mut self) {
        println!("Enter a todo item: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        self.todo_list.push(input.trim().to_string());
    }

    fn print_todo_list(&self) {
        if self.todo_list.is_empty() {
            println!("Your todo list is empty.");
        } else {
            println!("Your todo list: ");
            for (i, todo) in self.todo_list.iter().enumerate() {
                println!("{}: {}", i, todo);
            }
        }
    }

    fn edit_todo(&mut self) {
        self.print_todo_list();
        println!("Enter the number of the todo item you would like to edit: ");
        let mut todo_id = String::new();
        io::stdin()
            .read_line(&mut todo_id)
            .expect("Failed to read line");

        if let Ok(index) = todo_id.trim().parse::<usize>() {
            if index < self.todo_list.len() {
                println!("Enter the new todo item: ");
                let mut new_todo = String::new();
                io::stdin()
                    .read_line(&mut new_todo)
                    .expect("Failed to read line");

                self.todo_list[index] = new_todo.trim().to_string();
            } else {
                println!("Invalid todo item number.");
            }
        } else {
            println!("Invalid input for todo item number.");
        }
    }

    fn remove_todo(&mut self) {
        self.print_todo_list();
        println!("Enter the number of the todo item you would like to remove: ");
        let mut todo_id = String::new();
        io::stdin()
            .read_line(&mut todo_id)
            .expect("Failed to read line");

        if let Ok(index) = todo_id.trim().parse::<usize>() {
            if index < self.todo_list.len() {
                self.todo_list.remove(index);
            } else {
                println!("Invalid todo item number.");
            }
        } else {
            println!("Invalid input for todo item number.");
        }
    }
}

fn main() {
    let mut app = TodoApp::new();

    loop {
        println!("What would you like to do?");
        println!("[0]: Add a todo");
        println!("[1]: Edit todo");
        println!("[2]: Remove a todo");
        println!("[3]: View your todo list");
        println!("[4]: Exit");

        match app.read_input() {
            Action::Add => app.add_todo(),
            Action::Edit => app.edit_todo(),
            Action::Remove => app.remove_todo(),
            Action::View => app.print_todo_list(),
            Action::Exit => break,
            Action::Invalid => println!("Invalid choice. Please enter a valid option."),
        }
    }
}
