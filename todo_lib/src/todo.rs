    use rand::prelude::*;

    #[derive(Clone)]
    pub struct Todo {
        pub id: u64,
        pub name: String,
        pub description: String
    }
    
    impl Todo {
        pub fn new(name: String, description: String) -> Todo{
            Todo {
                id: generate_random_id(),
                name: name,
                description: description
            }
        }

        
        pub fn print_todo(&self) {
            println!("id: {id} | name: {name} | description: {desc}",id = self.id,name = self.name, desc = self.description)
        }

    }

    fn generate_random_id() -> u64{
        let mut rng = rand::thread_rng();
        let y: u64 = rng.gen(); // g
        y
    }




