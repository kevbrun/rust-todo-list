use todo_lib::todo::Todo;
use todo_lib::todo_list::TodoList;
use std::error::Error;
use std::{io::{self, Read}, process};
use std::io::Write; // <--- bring flush() into scope


fn main() {
  

    let mut todo_list = TodoList::new();


    println!("Welcome to your Todo list");
    loop {
        print_options();

    

        let user_input = read_option_input().unwrap();

        if user_input == -1 {
            println!("Invalid input. Please enter a number again!");
        }else if user_input == 1 {
            create_todo_from_user(&mut todo_list);
            print_todo_list_state(&todo_list);
    
        }else if user_input == 3 {
            println!("Stop program");
            process::exit(0);
        }

        

        io::stdout().flush().unwrap();

     
    }

}


fn print_todo_list_state(todo_list : &TodoList) {
    println!("----------- Current list state---------");
    todo_list.print_todo_list();
}


fn create_todo_from_user( todo_list : &mut TodoList)  -> () {
    let todo_name = get_user_input("Set todo name");
    let todo_desc =  get_user_input("Set description");
    todo_list.add_todo(Todo::new(todo_name,todo_desc));
    println!("Task added!");
}


fn get_user_input(text_to_print : &str) ->String {
    println!("{}",text_to_print);
    let mut buffer : String  = String::new();
    io::stdin().read_line(&mut buffer).expect("error: unable to read user input");
    let trimmed_buffer = buffer.trim().to_string();
    trimmed_buffer
}

fn read_option_input() -> Option<i32> {
    let user_input = get_user_input("Please enter choice:");
    
    match user_input.trim().parse() {
        Ok(value) => Some(value),
        Err(error) => Some(-1)
    }
}

fn print_options(){
    println!("What do you want to do!");
    println!("[1] Add Todo");
    println!("[2] Delete Todo");
    println!("[3] Stop program");
}

  


  
  

  