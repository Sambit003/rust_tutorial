static mut STATIC_ADDRESS: usize = 0;
//noinspection RsPlaceExpression
pub(crate) fn print_value() {
    let mut values = vec![1, 1, 1]; // vector allocated here
    for i in 3..10 {
        let next = values[i-3] + values[i-2];
        values.push(next);
    }
    println!("Values(1..10): {:?}", values);

    // capacity of the vector
    println!("Capacity: {}", values.capacity());
    // length of the vector
    println!("Length: {}", values.len());

    // Address of the vector buffer, capacity and length placeholders
    let vector_buffer_address = &values as *const _ as usize;
    let vector_capacity_address = &values.capacity() as *const _ as usize;
    let vector_length_address = &values.len() as *const _ as usize;
    println!("\nVector Buffer address :{:x}\nVector Capacity address :{:x}\nVector Length address :{:x}\n",
             vector_buffer_address, vector_capacity_address, vector_length_address);

    // Let's print out the memory address
    for (i, value) in values.iter().enumerate() {
        let address = value as *const _ as usize;
        // Assign the address of the first element to a static variable
        if i == 0 {
            unsafe {
                STATIC_ADDRESS = address;
                let value = STATIC_ADDRESS as *const i32;
                println!("Value at address {:x} (of first index): {}", STATIC_ADDRESS, *value);
            }
        }
        println!("Value at buffer index {}: {}, Address: {:x}", i, value, address);
    }
}

// Let's try to access the value of the first element using the static variable
pub(crate) fn access_static_address() {
    unsafe {
        let value = STATIC_ADDRESS as *const i32;
        println!("\n\n::OWNSERSHIP IN ACTION::\nValue at address {:x}: {}", STATIC_ADDRESS, *value); // Run the main.rs file to see the output
        /*
         vector variable "values" is only local to the print_value function and is dropped after the function returns.
         Means only at that time the values in the vector are wiped out when the variable got out of scope.
         No memory leak here.
         */

        // free the STATIC_ADDRESS
        STATIC_ADDRESS = 0;
    }
    // Why this is unsafe block in Rust?
    // The unsafe block is used to indicate that the code inside the block is playing with bare access concepts of memory space.
}