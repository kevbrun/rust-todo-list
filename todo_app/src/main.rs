use todo_lib::todo::Todo;
use todo_lib::todo_list::TodoList;
use std::{io::{self, Read}, process};


fn main() {
  

    let mut todo_list = TodoList::new();


    println!("Welcome to your Todo list");
    loop {
        print_options();

    

        let user_input = match read_option_input() {
            Ok(value)=> value,
            Err(error) => {
                -1
            }
        };

        if user_input == -1 {
            println!("Invalid input. Please enter a number again!");
        }else if user_input == 3 {
            println!("Stop program");
            process::exit(0);
        }



    }

}

fn read_option_input() -> Result<i32,&'static str> {
    let mut buffer : String  = String::new();
        let user_input =  match io::stdin().read_line(&mut buffer)
         {
             Err(error) => {
                String::from("")
             },
             _ => buffer
         };

        let number : Result<i32,&str> = match user_input.trim().parse() {
            Ok(v) => Ok(v),
            Err(error) => Err("Invalid input! Input is not a number!"),

        };

     number   
}


fn print_options(){
    println!("What do you want to do!");
    println!("[1] Add Todo");
    println!("[2] Delete Todo");
    println!("[3] Stop program");
}

  


  
  

  