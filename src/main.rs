mod shopping_list;
use shopping_list::ShoppingList;

#[derive(Debug)]
pub enum MyErrors {
    ParseIntErr(std::num::ParseIntError),
    StringTooShort,
    ItemRemovalErr,
    IoReadErr(std::io::Error),
}

impl From<std::num::ParseIntError> for MyErrors {
    fn from(error: std::num::ParseIntError) -> Self {
        MyErrors::ParseIntErr(error)
    }
}

impl From<std::io::Error> for MyErrors {
    fn from(error: std::io::Error) -> Self {
        MyErrors::IoReadErr(error)
    }
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
        std::io::stdin()
            .read_line(&mut selection)
            .map_err(MyErrors::IoReadErr)?;

        match selection.trim().parse::<i32>() {
            Ok(1) => { if let Err(MyErrors::StringTooShort) = shopping_list.add_item() {
                println!("Cannot add empty string or string too short");
                         continue;
            }},

            Ok(2) => {
                shopping_list.show_list();
            },
            
            Ok(3) => {
                if let Err(MyErrors::ItemRemovalErr) = shopping_list.remove_item() {
                    println!("Nothing to remove!"); 
                    continue;
                };
            },

            Ok(4) => {
                println!("NOT IMPLEMENTED YET!");
                continue;
            },

            Ok(5) => {
                println!("Quitting program");
                program_running = false;
            },
            
            Err(e) => {
                println!("Select between 1-5. \nError was: {}", e);
            },
            
            _ => {
                println!("Select between 1-5");
                continue;
            },
        }
    }
    Ok(())
}
