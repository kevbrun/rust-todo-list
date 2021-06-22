pub enum Choice {
    Add(1,"Add Todo"),
    Delete(2,"Delete todo"),
    Print(3,"Print list"),
    Quit(4,"Quit program"),
    NA(99,"N")
}


impl Choice {

    fn parse(choice: i32) -> Choice {
        match choice {
            1 => Add,
            2 => Delete,
            3 => Print,
            4 => Quit,
            _ => NA

        }
    }
}