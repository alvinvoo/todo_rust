/*
 * Requirements:
 * 1. When fired up, display stored tasks from a csv file if there's any
 * 2. Type 'a' and [enter] to enter new task mode
 * 3. Type the said task, press [enter] to save task 
 * 4. ToDo list will be refreshed and display all tasks
 * 5. Type 'd' and [enter] to enter delete mode
 * 6. Type the said task no., press [enter] to delete task
 * 7. No. 4
 * 8. Type 's' and [enter] to save ToDo list to csv file
 * 8. Type 'e' and [enter] to exit program, the existing ToDo list will be saved into a csv file
 * 9. Type 't' and [enter] to enter toggle mode
 * 10. Type the said task no., press [enter] to toggle complete/incomplete
 */
use std::io;
use std::io::ErrorKind;
use std::convert::TryInto;
use std::fs::File;
use std::fs;
use std::process;

#[derive(Debug)]
struct ToDo {
    task: String,
    completed: bool
}

impl ToDo {
    fn new(task: &str, completed: bool) -> ToDo {
        ToDo{
            task: String::from(task),
            completed
        }
    }
}

fn print_list(list: &Vec<ToDo>) -> String{
    let iter = list.iter().enumerate();
    let mut final_str = String::new();

    for (ind, val) in iter {
        let t_str = format!("{}, {}, {}", (ind + 1), val.task, val.completed);
        final_str = format!("{}\n{}", final_str, t_str);
    }

    final_str
}

fn create_from_file(file_content: &str, todo_list: &mut Vec<ToDo>){
    let task_iter = file_content.split('\n');

    for t in task_iter {
        let t_i: Vec<&str> = t.split(',').collect();

        if t_i.len() != 3 {
            continue;
        }

        let t_completed: bool  = t_i[2].trim().parse().unwrap();

        let todo = ToDo::new(t_i[1].trim(), t_completed);
        todo_list.push(todo);
    }
}

fn main() {
    let fs_str = fs::read_to_string("todo.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("todo.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            });
            String::new()
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    let mut todo_list = vec![];

    create_from_file(&fs_str, &mut todo_list);

    loop {
        println!("\n--- ToDo List ---");
        let list = print_list(&todo_list);
        println!("{}", list);
        println!("--- end --- \n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "a" | "A" => {
                println!("Add one task!");
                let mut addition = String::new();
                io::stdin()
                    .read_line(&mut addition)
                    .expect("Failed to add!");
                todo_list.push(ToDo::new(addition.trim(), false));
            },
            "d" | "D" => {
                println!("Delete one task!");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to delete!");

                let index: u32 = match index.trim().parse::<u32>() {
                    Ok(num) => {
                        let list_len = todo_list.len() as u32;

                        if num < 1 || num > list_len {
                            println!("Task index not found!");
                            continue;
                        } else { num - 1 }
                    },
                    Err(e) => {
                        println!("Error {:?}", e);
                        continue;
                    }
                };

                todo_list.remove(index.try_into().unwrap()); 
            },
            "t" | "T" => {
                println!("Toggle one task!");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to delete!");

                let index: u32 = match index.trim().parse::<u32>() {
                    Ok(num) => {
                        let list_len = todo_list.len() as u32;

                        if num < 1 || num > list_len {
                            println!("Task index not found!");
                            continue;
                        } else { num - 1 }
                    },
                    Err(e) => {
                        println!("Error {:?}", e);
                        continue;
                    }
                };

                let mut this_todo = &mut todo_list[index as usize];
                this_todo.completed = !this_todo.completed;
            },
            "s" | "S" => {
                println!("Saving todo list!");
                fs::write("todo.txt", print_list(&todo_list)).expect("Problem saving todo list file");
            },
            "e" | "E" => {
                println!("Saving and exiting! Bye!");
                fs::write("todo.txt", print_list(&todo_list)).expect("Problem saving todo list file");

                process::exit(0);
            },
            _ => todo_list.push(ToDo::new(input.trim(), false)),
        }
    }
}
