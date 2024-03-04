Rusty Store Inventory Management System
Overview
The Rusty Store Inventory Management System is a command-line application designed for small retail stores to manage their inventory efficiently. This system allows store managers to add, edit, and delete products from the inventory, generate reports, and perform basic authentication for security.

Features
Inventory Management: Add, edit, and delete products from the inventory.
Reporting: Generate reports displaying the store's inventory.
Error Handling: Robust error handling for invalid inputs and other errors.
Security: Basic authentication to prevent unauthorized access.
User Interface: Clear and intuitive text-based user interface.
Installation
Clone the repository: git clone <repository_url>
Navigate to the project directory: cd rusty-store-inventory
Compile the code: cargo build
Usage
To run the Rusty Store Inventory Management System:

bash
Copy code
cargo run
Commands
Add Product: Add a new product to the inventory.
Edit Product: Modify existing product details.
Delete Product: Remove a product from the inventory.
Generate Report: View the store's inventory report.
Exit: Quit the application.
Examples
Adding a Product:
Enter product details prompted by the system.
Editing a Product:
Enter the name of the product to edit, choose the field to modify, and provide the new value.
Deleting a Product:
Enter the name of the product to delete.
Generating a Report:
View the inventory report displayed by the system.
Testing
The project includes a suite of test cases to ensure the correctness of the code. To run the tests:

bash
Copy code
cargo test
Contributing
Contributions are welcome! If you'd like to contribute to the project, please open an issue or submit a pull request.

License
This project is licensed under the MIT License - see the LICENSE file for details.
