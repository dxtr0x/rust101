use crud::{create_database, Menu};

fn main(){
    if let Some(db) = create_database() {
            Menu::menu_note_selection(&db);
    }
    println!("\n👋👋 See ya !! 👋👋\n");
}