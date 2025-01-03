/*
This example demonstrates how comparison operators work with references in Rust,
illustrating that they compare the underlying values, not the reference addresses
themselves. It also shows how to compare reference addresses explicitly using
`std::ptr::eq`. This distinction is important for understanding Rust's behavior
when working with references.
*/

pub(crate) fn comparing_references_demo() {
    println!("Demonstrating Comparing References in Rust:");

    let a = 5;
    println!("Memory address of a: {:p}", &a);
    let b = 5;
    println!("Memory address of b: {:p}", &b);

    let ref_a = &a;
    println!("Memory address of ref_a: {:p}", ref_a);
    let ref_b = &b;
    println!("Memory address of ref_b: {:p}", ref_b);

    let ref_ref_a = &ref_a;
    println!("Memory address of ref_ref_a: {:p}", ref_ref_a);
    let ref_ref_b = &ref_b;
    println!("Memory address of ref_ref_b: {:p}", ref_ref_b);

    // **1. Comparing Values Through References**
    println!("\n- Comparing Values Through References:");
    assert_eq!(ref_a, ref_b);
    println!("  ref_a == ref_b: {}", ref_a == ref_b); // Compares the values of a and b

    assert_eq!(ref_ref_a, ref_ref_b);
    println!("  ref_ref_a == ref_ref_b: {}", ref_ref_a == ref_ref_b); // Compares the values of a and b

    // **2. Comparing Reference Addresses**
    println!("\n- Comparing Reference Addresses:");
    assert!(!std::ptr::eq(ref_a, ref_b));
    println!("  Memory addresses of ref_a and ref_b are the same: {}", std::ptr::eq(ref_a, ref_b));

    assert!(!std::ptr::eq(ref_ref_a, ref_ref_b));
    println!("  Memory addresses of ref_ref_a and ref_ref_b are the same: {}", std::ptr::eq(ref_ref_a, ref_ref_b));

    // **3. Comparing References of Different Levels**
    println!("\n- Comparing References of Different Levels:");
    // Error: mismatched types `&i32` and `&&i32`
    // assert_eq!(ref_a, ref_ref_a);

    assert_eq!(ref_a, *ref_ref_a);
    println!("  ref_a == *ref_ref_a: {}", ref_a == *ref_ref_a); // Comparing &i32 with &i32

    // Example with a custom struct
    #[derive(PartialEq, Debug)]
    struct Point { x: i32, y: i32 }

    let p1 = Point { x: 1, y: 2 };
    println!("Memory address of p1: {:p}", &p1);
    let p2 = Point { x: 1, y: 2 };
    println!("Memory address of p2: {:p}", &p2);

    let ref_p1 = &p1;
    println!("Memory address of ref_p1: {:p}", ref_p1);
    let ref_p2 = &p2;
    println!("Memory address of ref_p2: {:p}", ref_p2);

    println!("\n- Comparing custom struct:");
    assert_eq!(ref_p1, ref_p2);
    println!("  ref_p1 == ref_p2: {}", ref_p1 == ref_p2); // Compares the fields of Point

    assert!(!std::ptr::eq(ref_p1, ref_p2));
    println!("  Memory addresses of ref_p1 and ref_p2 are the same: {}", std::ptr::eq(ref_p1, ref_p2));
}