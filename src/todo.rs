    #[derive(Clone)]
    pub struct Todo {
        pub id: u64,
        pub name: String,
        pub description: String
    }
    
    impl Todo {
        pub fn new(id:u64, name: String, description: String) -> Todo{
            Todo {
                id: id,
                name: name,
                description: description
            }
        }

        
        pub fn print_todo(&self) {
            println!("id: {id} | name: {name} | description: {desc}",id = self.id,name = self.name, desc = self.description)
        }

    }




