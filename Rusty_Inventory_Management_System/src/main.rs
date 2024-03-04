use std::io::{self, Write};

struct Inventory{
    products: Vec<Product>,
}
struct Product{
    name: String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Inventory{
 fn add_product(&mut self,product: Product){
         self.products.push(product);
 }
 
 fn delete_product(&mut self, name: &str) {
    if let Some(index) = self.products.iter().position(|p| p.name == name) {
        self.products.remove(index);
        println!("Product '{}' deleted successfully!", name);
    } else {
        println!("Product '{}' not found.", name);
    }
}

fn edit_product(&mut self, name: &str, field: &str, value: &str) {
    if let Some(product) = self.products.iter_mut().find(|p| p.name == name) {
        match field {
            "name" => product.name = value.to_string(),
            "description" => product.description = value.to_string(),
            "price" => {
                if let Ok(price) = value.parse::<f64>() {
                    product.price = price;
                } else {
                    println!("Invalid price format.");
                    return;
                }
            }
            "quantity" => {
                if let Ok(quantity) = value.parse::<i32>() {
                    product.quantity = quantity;
                } else {
                    println!("Invalid quantity format.");
                    return;
                }
            }
            _ => {
                println!("Invalid field.");
                return;
            }
        }
        println!("Product edited successfully!");
    } else {
        println!("Product not found.");
    }
}


fn generate_report(&self) {
    println!("Inventory Report:");
    println!("Name\tDescription\tPrice\tQuantity");
    for product in &self.products {
        println!(
            "{}\t{}\tRs{:.2}\t{}",
            product.name, product.description, product.price, product.quantity
        );
    }
}


}


fn main() {
    let mut inventory = Inventory { products: Vec::new() };

    loop {
        println!("\nMenu:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                // Add Product
                println!("Enter product details:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                print!("Description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");

                print!("Price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin().read_line(&mut price).expect("Failed to read line");
                let price: f64 = price.trim().parse().expect("Invalid price");

                print!("Quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).expect("Failed to read line");
                let quantity: i32 = quantity.trim().parse().expect("Invalid quantity");

                inventory.add_product(Product {
                    name: name.trim().to_string(),
                    description: description.trim().to_string(),
                    price,
                    quantity,
                });
                println!("Product added successfully!");
            }
            2 => {
                // Edit Product
                // Edit Product
                println!("Enter the name of the product to edit:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");

                println!("Enter the field to edit (name, description, price, quantity):");
                print!("Field: ");
                io::stdout().flush().unwrap();
                let mut field = String::new();
                io::stdin()
                    .read_line(&mut field)
                    .expect("Failed to read line");

                println!("Enter the new value:");
                print!("Value: ");
                io::stdout().flush().unwrap();
                let mut value = String::new();
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read line");

                inventory.edit_product(name.trim(), field.trim(), value.trim());
            }
            3 => {
                // Delete Product
                println!("Enter the name of the product to delete:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");

                inventory.delete_product(name.trim());
            }
            4 => {
                // Generate Report
                inventory.generate_report();
            }
            5 => {
                // Exit
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }

  

}
