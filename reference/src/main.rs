use std::io::{self, Write};
use std::process::Command;

mod reference_example;
mod ref_deref_demo;
mod ref_assignment_demo;
mod ref_to_ref;
mod ref_compare_demo;
mod refs_are_not_null_demo;
mod ref_to_expression;

fn main() {
    loop {
        let menu_items = [
            "Reference Example Demo",
            "Working with References Demo",
            "Assigning References Demo",
            "References to References Demo",
            "Comparing References Demo",
            "References are Not Null Demo",
            "Borrowing References to Arbitrary Expressions Demo",
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
                println!("--------STARTING OF REFERENCE EXAMPLE DEMO--------");
                reference_example::reference_example();
                println!("--------END OF REFERENCE EXAMPLE DEMO--------\n");
            }
            2 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF WORKING WITH REFERENCES DEMO--------");
                ref_deref_demo::working_with_references_demo();
                println!("--------END OF WORKING WITH REFERENCES DEMO--------");
            }
            3 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF ASSIGNING REFERENCES DEMO--------");
                ref_assignment_demo::assigning_references_demo();
                println!("--------END OF ASSIGNING REFERENCES DEMO--------");
            }
            4 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF REFERENCES TO REFERENCES DEMO--------");
                ref_to_ref::references_to_references_demo();
                println!("--------END OF REFERENCES TO REFERENCES DEMO--------");
            }
            5 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF COMPARING REFERENCES DEMO--------");
                ref_compare_demo::comparing_references_demo();
                println!("--------END OF COMPARING REFERENCES DEMO--------");
            }
            6 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF REFERENCES ARE NOT NULL DEMO--------");
                refs_are_not_null_demo::references_never_null_demo();
                println!("--------END OF REFERENCES ARE NOT NULL DEMO--------");
            }
            7 => {
                Command::new("clear").status().unwrap();
                println!("--------STARTING OF BORROWING REFERENCES TO ARBITRARY EXPRESSIONS DEMO--------");
                ref_to_expression::borrowing_references_to_arbitrary_expressions_demo();
                println!("--------END OF BORROWING REFERENCES TO ARBITRARY EXPRESSIONS DEMO--------");
            }
            8 => {
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