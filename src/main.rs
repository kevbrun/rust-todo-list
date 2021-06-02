mod todo;
mod todo_list;

use todo::Todo;
use todo_list::TodoList;


fn main() {
    let test_todo = Todo::create(1,String::from("TEST"),String::from("Test"));
    let mut todo_list = TodoList::create();
    
    todo_list.add_todo(test_todo);
    todo_list.add_todo(Todo {
        id: 0,
        name: String::from("Test Task"),
        description: String::from("Description")
    });

    todo_list.print_todo_list();

    println!("Try to search for todo");
    let found_todo = todo_list.get_todo_by_id(1);

    match found_todo {
        Some(todo) => todo.print_todo(),
        None => println!("Todo with was not found") 
    }

    println!("Print all again");
    todo_list.print_todo_list();

    println!("Delete at pos 2");
    todo_list.delete_todo_by_id(2);

    println!("Print all again");
    todo_list.print_todo_list();

}

  


  
  

  