/*
This example demonstrates basic reference usage in Rust. While it's a straightforward
illustration, later examples will build upon these concepts to explore more complex
scenarios involving lifetimes, borrowing, and mutation. So, while this file might
seem simple, it lays the foundation for deeper understanding in the following examples.
*/

use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {  // Accepts a shared reference to the table
    println!("Table address inside show: {:p}", table);  //:p is format specifier to print the pointer value in hex format

    for (artist, works) in table {
        println!("Works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) { // Accepts a mutable reference
    println!("Table address inside sort_works: {:p}", table);

    for (_artist, works) in table {
        works.sort();
    }
}

pub(crate) fn reference_example() {

    println!("Demonstrating References in Rust:");

    // Create the table. We'll keep track of its memory address.
    let mut table = Table::new();
    println!("Table address in main: {:p}", &table);

    table.insert(
        "Da Vinci".to_string(),
        vec![
            "Mona Lisa".to_string(),
            "The Last Supper".to_string(),
            "Vitruvian Man".to_string(),
        ],
    );
    table.insert(
        "Michelangelo".to_string(),
        vec![
            "David".to_string(),
            "Sistine Chapel Ceiling".to_string(),
            "Pietà".to_string(),
        ],
    );

    // Passing a shared reference:
    show(&table);  // Ownership is *not* transferred. The table variable remains usable after this call.

    // Print the table after `show` to prove it's still accessible.
    println!("Original table still exists and is accessible in main().");

    println!("\nSorting the works now:");

    // Passing a mutable reference:
    sort_works(&mut table);

    println!("\nDisplaying the table after sorting:");
    show(&table);

    println!("\nAccessing the sorted values through index access:");
    // Now you can access the sorted value through indexing since `show()` returns ownership to `table`.
    let davincis_work = &table["Da Vinci"]; // Returns a reference to value, not move or copy
    for masterpiece in davincis_work{
        println!("{}", masterpiece);

    }
    println!("Still davincis works are accessible through index in the table: {}", &table["Da Vinci"][2]); // Value still there

}