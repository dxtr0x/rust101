use rusqlite::{Connection};
use std::io::{self, Write};

pub enum Menu{
    Add = 1,
    Delete = 2,
    Show = 3,
    Exit = 4
}

// struct note(database_seed,id,note)
#[derive(Debug)]
pub struct Note{
    id : i32,
    note : String
}
// Input function 

pub fn input() -> String{
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .unwrap();
    inp
}

pub fn input_menu() -> Menu {
    print!("🚀 Enter your choice : ");
    io::stdout().flush().unwrap();
    let int = input();
    let input = int.trim().parse::<i32>().unwrap();
    match input {
            1 => Menu::Add,
            2 => Menu::Delete,
            3 => Menu::Show,
            4 => Menu::Exit,
            _ => Menu::Exit
    }
}

pub fn input_int() -> i32 {
    print!("🚀🚀 Enter your choice : ");
    io::stdout().flush().unwrap();
    let int = input();
    let input = int.trim().parse::<i32>().unwrap();
    input
}

pub fn input_note() -> String {
    print!("🚀🚀 Enter your note : ");
    io::stdout().flush().unwrap();
    input()
}

// function to create database for notes
pub fn create_database() -> Option<Connection>{
    match Connection::open_in_memory() {
        Ok(conn) => {
            match conn.execute("CREATE TABLE IF NOT EXISTS Note (id INTEGER PRIMARY KEY AUTOINCREMENT, note TEXT NOT NULL);", ())
                {
                    Ok(_) => Some(conn),
                    Err(_) => None
                }
        },
        Err(_) => None
    }
}

pub fn add_note(db:&Connection , msg : &str){
    db.execute(
        "Insert into Note(note) values (?1) ", 
        (&msg,)
    ).expect("Problem in execute insert note !!!");
}

pub fn show_notes(db:&Connection){

    let mut stmt = db.prepare(
        "Select * from Note;"
    ).expect("Problem execute Select command to show all notes");

    let note_iter = stmt.query_map([], |row| {
        Ok(Note{
            id : row.get(0)?,
            note : row.get(1)?
        })
    }).expect("Failed to map notes");

    println!("\n\x1B[32m======================= Notes =============================");

    for note in note_iter {
        let note = note.expect("Failed to get id");
        println!("# {:?} => {:?}", &note.id, &note.note);
    }

    println!("==========================================================\x1B[0m\n");

}

pub fn delete_note(db:&Connection,id : &i32){
    db.execute(
        "Delete from Note where id = ?1 ;",(&id,)
    ).expect("Error when delete note");
}

impl Menu {
    pub fn menu_note_selection(db:&Connection){
        loop {
            println!("\n\x1B[31m===================== Note App =====================");
            println!("1 - Add Note ");
            println!("2 - Delete Note ");
            println!("3 - Show Notes");
            println!("4 - Exit \x1B[0m");
            let input_t = input_menu();
            match input_t{
                Menu::Add => add_note(db, &input_note()),
                Menu::Delete => delete_note(db, &input_int()),
                Menu::Show => show_notes(db),
                Menu::Exit => break
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
