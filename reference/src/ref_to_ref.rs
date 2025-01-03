/*
This example demonstrates references to references in Rust, illustrating how multiple
levels of indirection work. While this might seem like a basic concept, understanding
how Rust handles nested references is crucial for more advanced topics like borrowing
and lifetime management. Consider this a stepping stone to more complex ideas.
*/

#[derive(Debug)]
struct Data {
    value: i32,
}

pub(crate) fn references_to_references_demo() {
    println!("Demonstrating References to References in Rust:");

    let data = Data { value: 42 };
    println!("Memory address of data: {:p}", &data);

    // Single reference
    let r: &Data = &data;
    println!("Memory address of reference r: {:p}", r);
    println!("Value through r: {}", r.value); // Implicit dereference

    // Reference to a reference
    let rr: &&Data = &r;
    println!("Memory address of reference rr: {:p}", rr);
    println!("Memory address rr points to: {:p}", *rr);
    println!("Value through rr: {}", rr.value); // Implicit dereference

    // Reference to a reference to a reference
    let rrr: &&&Data = &rr;
    println!("Memory address of reference rrr: {:p}", rrr);
    println!("Memory address rrr points to: {:p}", *rrr);
    println!("Memory address pointed to by what rrr points to: {:p}", **rrr);
    println!("Value through rrr: {}", rrr.value); // Implicit dereference

    // Demonstrating implicit dereferencing with the . operator
    println!("\nDemonstrating Implicit Dereferencing with the . Operator:");
    assert_eq!(rrr.value, 42);
    println!("Accessing value through rrr.value (implicit dereference): {}", rrr.value);

    // Let's modify the data and see the change through the references
    let mut mutable_data = Data { value: 100 };
    println!("Memory address of mutable_data: {:p}", &mutable_data);

    let mut r_mut: &mut Data = &mut mutable_data;
    println!("Memory address of mutable reference r_mut: {:p}", r_mut);

    let rr_mut: &mut &mut Data = &mut r_mut;
    println!("Memory address of mutable reference rr_mut: {:p}", rr_mut);

    // To modify the value, we need to dereference appropriately
    **rr_mut = Data { value: 200 }; // Need two dereferences

    println!("Modified value through rr_mut: {}", mutable_data.value);
}