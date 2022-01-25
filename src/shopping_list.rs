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
                println!("{}", item.trim());
            }
        }
    }

    pub fn add_item(&mut self) -> Result<&mut Self, MyErrors> {
        println!("Enter the item to be added to the list");

        let mut item = String::new();
        std::io::stdin().read_line(&mut item)?;

        if item == "\n" || item.len() <= 3 { 
            Err(MyErrors::StringTooShort)  
        }

        else {
            self.items.push(item);

            Ok(self)
        }
    }

    pub fn remove_item(&mut self) -> Result<&mut Self, MyErrors> {

        if self.items.len() == 0 {
            Err(MyErrors::ListEmpty)
        }

        else {
            println!("Enter number of item to be removed from list:\n");
            self.show_list();

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection)?;

            let selection = selection.trim().parse::<usize>()?;
            if selection < self.items.len() {
                self.items.remove(selection);
                Ok(self)
            } 
            else {
                return Err(MyErrors::IterOutOfBounds);
            }
        }
        
    }
}

impl std::fmt::Display for ShoppingList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({:?})", self.items)
    }

}
