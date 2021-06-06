use crate::todo::Todo;
use json;


pub struct TodoList {
    pub list: Vec<Todo>
}

impl TodoList {

    pub fn new() -> TodoList {
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
        self.list.iter().for_each(|todo| todo.print_todo())
     }

     pub fn print_todo_list_reverse(&self) {
         self.list.iter().rev().for_each(|todo| todo.print_todo())
     }

    pub fn get_todo_by_id(&self, id : u64) -> Option<&Todo> {
    
        for element in self.list.iter(){
            if element.id == id {
               return Some(&element);
            
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn delete_todo_by_id(&mut self, id : u64)  {
        let element = self.get_todo_by_id(id);
        
        let result = match element {
            Some(x) => x.id,
            None => 0 as u64,
        };

        if result == 0 {
            println!("Item was not found!");
           
        }else {
            let result = (result -1) as usize;
            println!("Remove item at position {}",result);
            self.list.remove(result);
        }  
    }
    

}
