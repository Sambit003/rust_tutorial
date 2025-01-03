/*
This example demonstrates Rust's ability to create references to the results
of arbitrary expressions, including function calls and intermediate values.
It illustrates how Rust manages the lifetimes of these anonymously created
temporary values to ensure memory safety.
*/

fn calculate_sum_of_squares(a: i32, b: i32) -> i32 {
    a * a + b * b
}

pub(crate) fn borrowing_references_to_arbitrary_expressions_demo() {
    println!("Demonstrating Borrowing References to Arbitrary Expressions:");

    // **1. Referencing the Result of a Function Call**
    println!("\n- Referencing the Result of a Function Call:");
    let ref_result = &calculate_sum_of_squares(3, 4);
    println!("  Reference to the result: {}", ref_result);
    println!("  Memory address of the temporary result: {:p}", ref_result);
    assert_eq!(*ref_result, 25);

    // The temporary holding the result lives as long as `ref_result`.

    // **2. Referencing an Intermediate Calculation**
    println!("\n- Referencing an Intermediate Calculation:");
    let x = 10;
    let ref_intermediate = &(x * 2 + 5);
    println!("  Reference to the intermediate calculation: {}", ref_intermediate);
    println!("  Memory address of the temporary intermediate value: {:p}", ref_intermediate);
    assert_eq!(*ref_intermediate, 25);

    // The temporary holding the intermediate value lives as long as `ref_intermediate`.

    // **3. Arithmetic Operations with References**
    println!("\n- Arithmetic Operations with References:");
    let y = 7;
    let ref_y = &y;
    let sum = ref_result + ref_y; // Rust dereferences implicitly for arithmetic
    println!("  Sum of reference and value: {}", sum);
    assert_eq!(sum, 32);

    // **4. Scope and Lifetime of Temporary References**
    println!("\n- Scope and Lifetime of Temporary References:");
    let value_in_statement = {
        let temp = &{ 5 * 5 }; // Anonymous temporary lives to the end of this block
        println!("  Reference to temporary inside block: {}", temp);
        println!("  Memory address of temporary inside block: {:p}", temp);
        *temp
    };
    println!("  Value obtained from temporary: {}", value_in_statement);
    assert_eq!(value_in_statement, 25);

    // **5. Example with String**
    println!("\n- Example with String:");
    let greeting = "Hello".to_string();
    let ref_len = &greeting.len();
    println!("  Reference to the length of the string: {}", ref_len);
    assert_eq!(*ref_len, 5);

    // **6. Demonstrating the Lifetime Extension**
    println!("\n- Demonstrating the Lifetime Extension:");
    let ref_to_temp;
    {
        let temp_value = & (10 * 3); // Temporary value
        ref_to_temp = temp_value;    // Lifetime of temp_value is extended
        println!("  Reference inside block: {}", ref_to_temp);
    }
    println!("  Reference outside block (lifetime extended): {}", ref_to_temp);
    assert_eq!(*ref_to_temp, 30);

    // **7. Example that would fail (demonstrating safety)**
    // The following would cause a compile error due to dangling reference:
    // let dangling_ref;
    // {
    //     let temp = &(50);
    //     dangling_ref = temp;
    // }
    // println!("Dangling ref: {}", dangling_ref); // Error: `temp` does not live long enough
}