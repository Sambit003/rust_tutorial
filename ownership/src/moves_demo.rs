/*
In this file, I will demonstrate you, how the ownership of variable assignment works in Rust.
With each assignment, the ownership of the variable is moved to the new variable. Let's see how it works.
 */

pub(crate) fn moves_demo() {
    let string_vector = vec!["Hello".to_string(), "World".to_string()];
    //let's inspect the memory of string_vector
    println!("string_vector buffer address: {:x} , value: {:?}\nString Vector capacity address: {:x} , value: {}\nString Vector length address: {:x} , value: {}",
             &string_vector as *const _ as usize, string_vector,
             &string_vector.capacity() as *const _ as usize, string_vector.capacity(),
             &string_vector.len() as *const _ as usize, string_vector.len());
    for (i, string) in string_vector.iter().enumerate() {
        println!("String at buffer index {}: {:?}", i, string);
        // Address of the String struct members and respective heap
        let string_address = string as *const _ as usize;
        println!("String address: {:x}, value: {}\nString Capacity Address: {:x}, value: {}\nString length address: {:x}, value: {}\n",
                 string_address, string,
                 &string.capacity() as *const _ as usize, string.capacity(),
                 &string.len() as *const _ as usize, string.len());
    }
    
    // Now assigning the value of string_vector to string_vector2
    let string_vector2 = string_vector;
    
    // Now assigning the value of string_vector to string_vector3
    // let mut string_vector3 = string_vector; // This line will throw an error....uncomment this line to see the error.
    
    // Let's inspect the memory of string_vector2 now
    println!("string_vector2 buffer address: {:x} , value: {:?}\nString Vector2 capacity address: {:x} , value: {}\nString Vector2 length address: {:x} , value: {}",
             &string_vector2 as *const _ as usize, string_vector2,
             &string_vector2.capacity() as *const _ as usize, string_vector2.capacity(),
             &string_vector2.len() as *const _ as usize, string_vector2.len());
    for (i, string) in string_vector2.iter().enumerate() {
        println!("String at buffer index {}: {:?}", i, string);
        // Address of the String struct members and respective heap
        let string_address = string as *const _ as usize;
        println!("String address: {:x}, value: {}\nString Capacity Address: {:x}, value: {}\nString length address: {:x}, value: {}\n",
                 string_address, string,
                 &string.capacity() as *const _ as usize, string.capacity(),
                 &string.len() as *const _ as usize, string.len());
    } // Now you can see that the string_vector2 has the ownership of the string_vector data. And the string_vector is no longer available. So you can't assign the value of string_vector to another variable henceforth.
    
    let string_vector3 = string_vector2; // This one is okay....but immediately after this line, the string_vector2 is no longer available.
    // At any point of time, just you'll not be able to find any memory trace of the string_vector and string_vector2 variables. They are just gone...I repeat, they are gone.
    // Let's inspect the memory of string_vector3 now
    println!("string_vector3 buffer address: {:x} , value: {:?}\nString Vector3 capacity address: {:x} , value: {}\nString Vector3 length address: {:x} , value: {}",
             &string_vector3 as *const _ as usize, string_vector3,
             &string_vector3.capacity() as *const _ as usize, string_vector3.capacity(),
             &string_vector3.len() as *const _ as usize, string_vector3.len());
    for (i, string) in string_vector3.iter().enumerate() {
        println!("String at buffer index {}: {:?}", i, string);
        // Address of the String struct members and respective heap
        let string_address = string as *const _ as usize;
        println!("String address: {:x}, value: {}\nString Capacity Address: {:x}, value: {}\nString length address: {:x}, value: {}\n",
                 string_address, string,
                 &string.capacity() as *const _ as usize, string.capacity(),
                 &string.len() as *const _ as usize, string.len());
    }
}

// Now, to if we want to have multiple copies, let's move to the next section
pub(crate) fn copy_demo(){
    let string_vector = vec!["Hello".to_string(), "World".to_string()];
    //let's inspect the memory of string_vector
    println!("string_vector buffer address: {:x} , value: {:?}\nString Vector capacity address: {:x} , value: {}\nString Vector length address: {:x} , value: {}",
             &string_vector as *const _ as usize, string_vector,
             &string_vector.capacity() as *const _ as usize, string_vector.capacity(),
             &string_vector.len() as *const _ as usize, string_vector.len());
    for (i, string) in string_vector.iter().enumerate() {
        println!("String at buffer index {}: {:?}", i, string);
        // Address of the String struct members and respective heap
        let string_address = string as *const _ as usize;
        println!("String address: {:x}, value: {}\nString Capacity Address: {:x}, value: {}\nString length address: {:x}, value: {}\n",
                 string_address, string,
                 &string.capacity() as *const _ as usize, string.capacity(),
                 &string.len() as *const _ as usize, string.len());
    }
    
    // Now assigning the value of string_vector to string_vector2
    let string_vector2 = string_vector.clone();
    
    // Now assigning the value of string_vector to string_vector3
    let string_vector3 = string_vector.clone();
    
    // Let's inspect the memory of string_vector2 now
    println!("string_vector2 buffer address: {:x} , value: {:?}\nString Vector2 capacity address: {:x} , value: {}\nString Vector2 length address: {:x} , value: {}",
             &string_vector2 as *const _ as usize, string_vector2,
             &string_vector2.capacity() as *const _ as usize, string_vector2.capacity(),
             &string_vector2.len() as *const _ as usize, string_vector2.len());
    for (i, string) in string_vector2.iter().enumerate() {
        println!("String at buffer index {}: {:?}", i, string);
        // Address of the String struct members and respective heap
        let string_address = string as *const _ as usize;
        println!("String address: {:x}, value: {}\nString Capacity Address: {:x}, value: {}\nString length address: {:x}, value: {}\n",
                 string_address, string,
                 &string.capacity() as *const _ as usize, string.capacity(),
                 &string.len() as *const _ as usize, string.len());
    }
    
    // Let's inspect the memory of string_vector3 now
    println!("string_vector3 buffer address: {:x} , value: {:?}\nString Vector3 capacity address: {:x} , value: {}\nString Vector3 length address: {:x} , value: {}",
             &string_vector3 as *const _ as usize, string_vector3,
             &string_vector3.capacity() as *const _ as usize, string_vector3.capacity(),
             &string_vector3.len() as *const _ as usize, string_vector3.len());
    for (i, string) in string_vector3.iter().enumerate() {
        println!("String at buffer index {}: {:?}", i, string);
        // Address of the String struct members and respective heap
        let string_address = string as *const _ as usize;
        println!("String address: {:x}, value: {}\nString Capacity Address: {:x}, value: {}\nString length address: {:x}, value: {}\n",
                 string_address, string,
                 &string.capacity() as *const _ as usize, string.capacity(),
                 &string.len() as *const _ as usize, string.len());
    }
}
