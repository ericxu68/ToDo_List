# Rust Checklist Application
This is a simple command-line checklist application written in Rust that allows users to add, remove, and modify items on their to-do list, and receive notifications for items that are due in the next hour.

## Getting Started
1. Make sure you have Rust installed on your system. If you don't have it installed, you can download it from the official Rust website: https://www.rust-lang.org/tools/install.

2. Clone the repository to your local machine.

3. Navigate to the repository directory and run the following command to build the application:


Copy code
'''
cargo build --release
'''

4. Run the application with the following command:


Copy code
'''
cargo run --release
'''

## Usage
Once you have the application running, you can use the following commands to manage your to-do list:

- add <name> <year>-<month>-<day> <hour>:<minute>: Add a new item to the to-do list with the specified name and due date.

- remove <name>: Remove the item with the specified name from the to-do list.

- modify <name> <year>-<month>-<day> <hour>:<minute>: Modify the due date of the item with the specified name.

- list: Display all items on the to-do list.

- exit: Exit the application.

The application will automatically display items that are due in the next hour every 5 minutes.

## License
This project is licensed under the MIT License - see the LICENSE file for details.



