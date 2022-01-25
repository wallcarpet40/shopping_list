mod shopping_list;
use std::num::ParseIntError;

use shopping_list::ShoppingList;

#[derive(Debug)]
enum MyErrors {
    ParseInt(ParseIntError),
    StringTooShort,
    ItemRemovalErr,
    IoReadErr(std::io::Error),

}

fn print_menu() {
    println!();
    println!("1. Add item to shopping list.");
    println!("2. Show shopping list.");
    println!("3. Delete item from shopping list.");
    println!("4. Save shopping list to disk. NOT IMPLEMENTED YET");
    println!("5. Quit.");
    println!();
    println!("Select number");
}

fn main() -> Result<(), Box<MyErrors>> {

    let mut shopping_list = ShoppingList::new();
    let mut program_running: bool = true;

    while program_running {
        print_menu();
        
        let mut selection = String::new();
        std::io::stdin().read_line(&mut selection).map_err(MyErrors::IoReadErr)?;
            
        match selection.trim().parse::<i32>() {
            
            Ok(1) => { shopping_list.add_item()?; },
            Ok(2) => { shopping_list.show_list(); },
            Ok(3) => { shopping_list.remove_item()?; }
            Ok(5) => { println!("Quitting program"); program_running = false; }
            Err(e) => { println!("Select between 1-5. \nError was: {}", e); },
            _ => { println!("Select between 1-5"); continue; }
        }
        
    }
    Ok(())
}
