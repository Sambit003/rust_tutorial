/*
This example demonstrates the basics of working with references in Rust, focusing on
explicit and implicit referencing and dereferencing. While the concepts are fundamental
and may seem simple here, understanding them thoroughly is crucial for grasping
Rust's ownership and borrowing system, which will become more important in
subsequent, more complex examples.
*/

#[derive(Debug)]
struct Gadget {
    id: i32,
    name: String,
}

impl Gadget {
    // Method taking a shared reference (implicit borrow)
    fn describe(&self) {
        println!("  Gadget {{ id: {}, name: '{}' }}", self.id, self.name);
    }

    // Method taking a mutable reference (implicit borrow)
    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }
}

pub(crate) fn working_with_references_demo() {
    println!("Demonstrating Working with References in Rust:");

    let my_gadget = Gadget {
        id: 123,
        name: "Sprocket".to_string(),
    };
    println!("Original gadget: {:?}", my_gadget);
    println!("Memory address of original gadget: {:p}", &my_gadget);

    // **1. Explicit Shared Reference and Dereferencing**
    println!("\n- Explicit Shared Reference and Dereferencing:");
    let shared_ref = &my_gadget;
    println!("  Shared reference address: {:p}", shared_ref);
    println!(
        "  Accessing fields through explicit dereference (*shared_ref).id: {}",
        (*shared_ref).id
    );
    println!(
        "  Accessing fields through explicit dereference (*shared_ref).name: {}",
        (*shared_ref).name
    );

    // **2. Explicit Mutable Reference and Dereferencing**
    println!("\n- Explicit Mutable Reference and Dereferencing:");
    let mut mutable_gadget = Gadget {
        id: 456,
        name: "Widget".to_string(),
    };
    println!("  Original mutable gadget: {:?}", mutable_gadget);
    println!("  Memory address of original mutable gadget: {:p}", &mutable_gadget);

    let mutable_ref = &mut mutable_gadget;
    println!("  Mutable reference address: {:p}", mutable_ref);
    (*mutable_ref).id = 789; // Modify through explicit dereference
    (*mutable_ref).name = "Super Widget".to_string();
    println!("  Modified mutable gadget through dereference: {:?}", mutable_gadget);

    // **3. Implicit Dereferencing with the `.` Operator**
    println!("\n- Implicit Dereferencing with the `.` Operator:");
    println!("  Accessing fields through implicit dereference shared_ref.id: {}", shared_ref.id);
    println!("  Accessing fields through implicit dereference shared_ref.name: {}", shared_ref.name);

    // **4. Implicit Borrowing for Method Calls**
    println!("\n- Implicit Borrowing for Method Calls:");
    println!("  Calling method with implicit borrow (shared):");
    shared_ref.describe(); // `describe` takes `&self`, so `shared_ref` is implicitly borrowed

    println!("  Calling method with implicit borrow (mutable):");
    let mut another_gadget = Gadget { id: 101, name: "Thing".to_string() };
    println!("  Original another_gadget: {:?}", another_gadget);
    let another_gadget_mut_ref = &mut another_gadget;
    another_gadget_mut_ref.rename("New Thing".to_string()); // `rename` takes `&mut self`, implicitly borrowed
    println!("  Modified another_gadget after rename: {:?}", another_gadget);

    // **5. Combining Concepts with a Vector**
    println!("\n- Combining Concepts with a Vector:");
    let mut gadgets = vec![
        Gadget { id: 1, name: "Alpha".to_string() },
        Gadget { id: 2, name: "Beta".to_string() },
    ];
    println!("  Original vector of gadgets: {:?}", gadgets);

    // Iterate using a shared reference
    println!("  Iterating with shared references:");
    for gadget in &gadgets { // Implicitly creates a shared reference to each gadget
        println!("    Gadget in vector: {:?}", gadget); // Implicit dereference for printing
        gadget.describe(); // Implicit borrow for method call
    }

    // Iterate using mutable references to modify
    println!("  Iterating with mutable references to modify:");
    for gadget in &mut gadgets { // Implicitly creates a mutable reference to each gadget
        gadget.rename(format!("Modified {}", gadget.name)); // Implicit borrow for mutable method
    }
    println!("  Modified vector of gadgets: {:?}", gadgets);
}