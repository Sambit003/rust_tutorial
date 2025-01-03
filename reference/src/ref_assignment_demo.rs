/*
This example demonstrates how assigning to a Rust reference changes the value it points to,
unlike C++ references which cannot be reseated once initialized. While this concept might
seem basic, it's important for understanding how references work in Rust and how they
differ from their counterparts in other languages. This foundational knowledge will be
crucial in more complex scenarios we'll explore later.
*/

pub(crate) fn assigning_references_demo() {
    println!("Demonstrating Assigning References in Rust:");

    let x = 10;
    println!("Memory address of x: {:p}", &x);

    let y = 20;
    println!("Memory address of y: {:p}", &y);

    // Initially, r points to x
    let mut r = &x;
    println!("Initial memory address r points to: {:p}", r);
    assert_eq!(*r, 10);

    // Change where r points based on a condition
    let b = true; // Or false, to see the different outcome
    if b {
        r = &y;
        println!("Memory address r points to after reassignment: {:p}", r);
        assert_eq!(*r, 20);
    } else {
        println!("Memory address r points to (condition false): {:p}", r);
        assert_eq!(*r, 10);
    }

    assert!(
        *r == 10 && !b || *r == 20 && b,
        "r should point to x or y depending on the value of b"
    );

    println!("Final value r points to: {}", *r);
}