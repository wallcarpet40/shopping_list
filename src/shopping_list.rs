use crate::MyErrors;

#[derive(Debug)]
pub struct ShoppingList {
    items: Vec<String>,
}

impl ShoppingList {
    pub fn new() -> ShoppingList {
        ShoppingList { items: Vec::new() }
    }

    pub fn show_list(&self) {
        if self.items.len() == 0 {
            println!("List is empty!");
        } 
        
        else {
            println!("Items on list:\n");

            for item in &self.items {
                println!("{:?}", item.trim());
            }
        }
    }

    pub fn add_item(&mut self) -> Result<&mut Self, MyErrors> {
        println!("Enter the item to be added to the list");

        let mut item = String::new();
        std::io::stdin().read_line(&mut item).map_err(MyErrors::IoReadErr)?;

        if item == "\n" || item.len() <= 3 { 
            println!("Cannot add an empty line");
            Err(MyErrors::StringTooShort)  // Why can't I return an Err(self) or Err(Box<dyn std::error::Error>)
        }

        else {
            self.items.push(item);

            Ok(self)
        }
    }

    pub fn remove_item(&mut self) -> Result<&mut Self, MyErrors> {

        if self.items.len() == 0 {
            println!("List is empty!");
            Ok(self)
        }

        else {
            println!("Enter number of item to be removed from list:\n");
            self.show_list();

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).map_err(MyErrors::IoReadErr)?;

            let selection = selection.trim().parse::<i32>().map_err(MyErrors::ParseInt)?;
            self.items.remove(selection as usize);
            Ok(self)
        }
        
    }
}