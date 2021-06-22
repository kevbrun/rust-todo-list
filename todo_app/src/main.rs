use todo_lib::todo::Todo;
use todo_lib::todo_list::TodoList;
use std::error::Error;
use std::{io::{self, Read}, process};
use std::io::Write; // <--- bring flush() into scope
mod user_input;

fn main() {
  

    let mut todo_list = TodoList::new();


    println!("Welcome to your Todo list");
    loop {
        user_input::print_options();

    

        let user_input = user_input::read_option_input("Please select option").unwrap();

        if user_input == -1 {
            println!("Invalid input. Please enter a number again!");
        }else if user_input == 1 {
            user_input::create_todo_from_user(&mut todo_list);
            user_input::print_todo_list_state(&todo_list);
    
        }else if user_input == 3 {
            println!("Stop program");
            process::exit(0);
        }

        

        io::stdout().flush().unwrap();

     
    }

}

