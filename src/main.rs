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
        name: String::from("sdfjsldkfjslkadf"),
        description: String::from("lksdjflksdjflksdfjlksdjfsd")
    });

    todo_list.print_todo_list();

}

  


  
  

  