    use rand::prelude::*;
    use std::fmt;
    use std::fmt::Display;


    #[derive(Clone)]
    pub struct Todo {
        pub id: u64,
        pub name: String,
        pub description: String
    }

    impl fmt::Display for Todo{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"id: {id}\t|name: {name}\t| description: {desc}",id = self.id,name = self.name, desc = self.description)
       }
    }
    
    impl Todo {
        pub fn new(name: String, description: String) -> Todo{
            Todo {
                id: generate_random_id(),
                name: name,
                description: description
            }
        }
    }

    fn generate_random_id() -> u64{
        let mut rng = rand::thread_rng();
        let y: u64 = rng.gen(); // g
        y
    }




