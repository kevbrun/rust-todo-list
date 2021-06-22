use todo_lib::todo::Todo;
use todo_lib::todo_list::TodoList;
use std::io;

pub fn print_todo_list_state(todo_list : &TodoList) {
    println!("----------- Current list state---------");
    todo_list.print_todo_list();
}


pub fn create_todo_from_user( todo_list : &mut TodoList)  -> () {
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

pub fn read_option_input(text_to_print : &str) -> Option<i32> {
    let user_input = get_user_input(text_to_print);
    
    match user_input.trim().parse() {
        Ok(value) => Some(value),
        Err(error) => Some(-1)
    }
}

pub fn delete_task(todo_list: &mut TodoList) {
    let item_to_delete = read_option_input("Please enter the number to delete:").expect("Could not get number from input!");
    let item_to_delete = item_to_delete as u64;

    todo_list.delete_todo_by_id(item_to_delete);

}

pub fn print_options(){
    println!("What do you want to do!");
    println!("[1] Add Todo");
    println!("[2] Delete Todo");
    println!("[3] Stop program");
}




  
  

  