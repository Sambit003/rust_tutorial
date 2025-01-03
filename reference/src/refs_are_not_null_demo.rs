/*
This example demonstrates that Rust references are guaranteed to never be null,
unlike pointers in C and C++. It showcases the use of `Option<&T>` as the
idiomatic way to represent the possibility of a missing reference in Rust, and
how this approach ensures safety and explicitness.
*/

pub(crate) fn references_never_null_demo() {
    println!("Demonstrating That Rust References Are Never Null:");

    // **1. Basic References**
    println!("\n- Basic References:");
    let value = 123;
    let reference: &i32 = &value;
    println!("  Value: {}, Reference value: {}", value, reference);
    println!("  Memory address of value: {:p}", &value);
    println!("  Memory address of reference: {:p}", reference);

    // In Rust, you cannot directly create a null reference.
    // The following line would cause a compile error:
    // let null_ref: &i32 = 0x0 as &i32; // Error: casts are not free

    // **2. Using `Option<&T>` for Potentially Missing References**
    println!("\n- Using `Option<&T>` for Potentially Missing References:");
    let possible_reference: Option<&i32> = Some(&value);
    match possible_reference {
        Some(r) => {
            println!("  Option contains a reference: {}", r);
            println!("  Memory address of referenced value: {:p}", r);
        }
        None => println!("  Option does not contain a reference."),
    }

    let absent_reference: Option<&i32> = None;
    match absent_reference {
        Some(_) => println!("  This should not be printed."),
        None => println!("  Option correctly indicates the absence of a reference."),
    }

    // **3. `Option<&T>` and Memory Representation**
    // At the machine level, `None` for `Option<&T>` is often represented as a null pointer.
    // This is an optimization, and you should still treat `Option<&T>` as an enum.

    // Here's a conceptual way to think about it (implementation detail):
    // None -> 0x0 (null pointer)
    // Some(&value) -> Non-zero address of value

    // **4. Safety of `Option<&T>`**
    println!("\n- Safety of `Option<&T>`:");
    fn print_if_some(opt_ref: Option<&i32>) {
        match opt_ref {
            Some(val) => println!("  Value is: {}", val),
            None => println!("  No value to print."),
        }
    }

    print_if_some(Some(&value));
    print_if_some(None);

    // Attempting to use a potentially null reference without checking in C/C++
    // would lead to a crash. Rust's `Option<&T>` forces you to handle both cases.
}