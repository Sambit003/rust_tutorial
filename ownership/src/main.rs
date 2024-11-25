use std::io::{self, Write};
use std::process::Command;

mod value_ownership_demo;
mod box_generic_type_demo;
mod struct_ownership_demo;
mod moves_demo;
mod rc_arc_demo;

fn main() {
    loop {
        let menu_items = [
            "Value Ownership Demo",
            "Box<T> Type Ownership Demo", 
            "Struct Ownership Demo",
            "Moves Demo",
            "Copy Demo",
            "Moves and Control Flow Demo",
            "Moves on Indexed Content",
            "Copy Types Demo",
            "Refercence Counted Rc<T> and Atomic Reference Counted Arc<T> Types Demo",
            "Exit"
        ];

        println!("\nMenu:");
        for (i, item) in menu_items.iter().enumerate() {
            println!("{}. {}", i + 1, item);
        }

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF VALUE OWNERSHIP DEMO--------");
                value_ownership_demo::print_value();
                value_ownership_demo::access_static_address();
                println!("--------END OF VALUE OWNERSHIP DEMO--------\n");
            }
            2 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF BOX<T> TYPE OWNERSHIP DEMO--------");
                box_generic_type_demo::box_demo();
                println!("--------END OF BOX<T> TYPE OWNERSHIP DEMO--------");
            }
            3 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF STRUCT OWNERSHIP DEMO--------");
                struct_ownership_demo::struct_ownership_demo();
                println!("--------END OF STRUCT OWNERSHIP DEMO--------");
            }
            4 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF MOVES DEMO--------");
                moves_demo::moves_demo();
                println!("--------END OF MOVES DEMO--------");
            }
            5 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF COPY DEMO--------");
                moves_demo::copy_demo();
                println!("--------END OF COPY DEMO--------");
            }
            6 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF MOVES CONTROL FLOW DEMO--------");
                moves_demo::moves_and_control_flow_demo();
                println!("--------END OF MOVES CONTROL FLOW DEMO--------");
            }
            7 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF MOVES ON INDEXED CONTENT DEMO--------");
                moves_demo::indexed_moves_demo();
                println!("--------END OF MOVES ON INDEXED CONTENT DEMO--------");
            }
            8 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF COPY TYPES DEMO--------");
                moves_demo::copy_types_demo();
                println!("--------END OF COPY TYPES DEMO--------");
            }
            9 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF REFERENCE COUNTED RC<T> AND ATOMIC REFERENCE COUNTED ARC<T> TYPES DEMO--------");
                rc_arc_demo::rc_arc_demo();
                println!("--------END OF REFERENCE COUNTED RC<T> AND ATOMIC REFERENCE COUNTED ARC<T> TYPES DEMO--------");
            }
            10 => {
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