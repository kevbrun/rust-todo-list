use crate::todo::Todo;

pub struct TodoList {
    pub list: Vec<Todo>
}

impl TodoList {

    pub fn create() -> TodoList {
        TodoList {
            list : Vec::new()
        }
    }

    pub fn add_todo(&mut self,  todo : Todo){
        let mut current_count = self.list.len() as u64;
        current_count = current_count+1;
        self.list.push(Todo {
            id: current_count,
            ..todo
        });
    }

    
    pub fn print_todo_list(&self){
        for element in self.list.iter() {
            element.print_todo();
        }

     }
}
