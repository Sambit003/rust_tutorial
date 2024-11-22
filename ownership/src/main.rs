use std::io::{self, Write};
use std::process::Command;

mod value_ownership_demo;
mod box_generic_type_demo;
mod struct_ownership_demo;
mod moves_demo;

fn main() {
    loop {
        println!("\nMenu:");
        println!("1. Value Ownership Demo");
        println!("2. Box<T> Type Ownership Demo");
        println!("3. Struct Ownership Demo");
        println!("4. Moves Demo");
        println!("5. Copy Demo");
        println!("6. Moves and Control Flow Demo");
        println!("7. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                println!("--------STARTING OF VALUE OWNERSHIP DEMO--------");
                value_ownership_demo::print_value();
                value_ownership_demo::access_static_address();
                println!("--------END OF VALUE OWNERSHIP DEMO--------\n");
            }
            2 => {
                println!("--------STARTING OF BOX<T> TYPE OWNERSHIP DEMO--------");
                box_generic_type_demo::box_demo();
                println!("--------END OF BOX<T> TYPE OWNERSHIP DEMO--------");
            }
            3 => {
                println!("--------STARTING OF STRUCT OWNERSHIP DEMO--------");
                struct_ownership_demo::struct_ownership_demo();
                println!("--------END OF STRUCT OWNERSHIP DEMO--------");
            }
            4 => {
                println!("--------STARTING OF MOVES DEMO--------");
                moves_demo::moves_demo();
                println!("--------END OF MOVES DEMO--------");
            }
            5 => {
                println!("--------STARTING OF COPY DEMO--------");
                moves_demo::copy_demo();
                println!("--------END OF COPY DEMO--------");
            }
            6 => {
                println!("--------STARTING OF MOVES CONTROL FLOW DEMO--------");
                moves_demo::moves_and_control_flow_demo();
                println!("--------END OF MOVES CONTROL FLOW DEMO--------");
            }
            7 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
        loop {
            println!("Press 'C' to continue...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().eq_ignore_ascii_case("C") {
                break;
            } else {
                println!("Invalid input, please press 'C' to continue.");
            }
        }
        Command::new("clear").status().unwrap();
    }
}