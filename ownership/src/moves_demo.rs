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

fn some_condition() -> bool {
    false
}

fn consume_vector(v: Vec<i32>) {
    println!("Consumed vector: {:?}", v);
}

fn use_vector(v: Vec<i32>) {
    println!("Using vector: {:?}", v);
}

pub(crate) fn moves_and_control_flow_demo() {
    let x = vec![10, 20, 30];
    if some_condition() {
        consume_vector(x); // x is moved here
    } else {
        use_vector(x); // x is moved here
    }

    // Case 1: Uncommenting this block will cause a compile-time error
    /* while some_condition() {
        consume_vector(x); // x is moved here
    } */  // This block is also a bad idea because x is moved here for first iteration and in second iteration, it's just a void space

    // Case 2:
    // x is uninitialized here because it has been moved in both branches of if-else
    //use_vector(x); // Uncommenting this line will cause a compile-time error
    // to use the variable x again, you need to either clone it or reassign it with direct value assignment or thtough any user defined funtion.
}

pub(crate) fn indexed_moves_demo() {
    // Imagine a library managing books. Each book has a title and availability status.
    struct Book {
        title: String,
        available: bool,
    }

    // Create a vector of books
    let mut library = vec![
        Book {
            title: "The Lord of the Rings".to_string(),
            available: true,
        },
        Book {
            title: "The Hitchhiker's Guide to the Galaxy".to_string(),
            available: true,
        },
        Book {
            title: "Pride and Prejudice".to_string(),
            available: false, // This one is already checked out
        },
        Book {
            title: "1984".to_string(),
            available: true,
        },
    ];

    //  Attempting a direct move (like this) will fail because it would leave the
    //  vector in an undefined state, as the element at index 1 would be moved out
    //  and the vector wouldn't know how to handle the "gap".
    //
    //  let checked_out_book = library[1]; // This would cause: "cannot move out of index of `Vec<Book>`"

    //
    // Method 1: Pop from the end of the vector.
    //
    // This is useful when order doesn't matter, or when you're specifically
    // working with the "top" or "last" element in a collection like a stack.
    //
    if let Some(last_book) = library.pop() {
        println!(
            "Popped book: '{}'. Availability: {}.",
            last_book.title, last_book.available
        );
    } else {
        println!("The library is empty!");
    }
    // Library now has 3 books: "The Lord of the Rings", "The Hitchhiker's Guide to the Galaxy", "Pride and Prejudice".
    // Last book "1984" is moved outside vector and assigned to last_book

    //
    // Method 2: Swap-remove from a specific index.
    //
    // This works when you want to get an element from a particular position
    // but also don't want to leave a "hole" in the vector. It replaces the
    // removed element with the last element, maintaining a contiguous vector.
    //
    let checked_out_book = library.swap_remove(0); // Check out the first book
    println!(
        "Checked out: '{}'. Availability: {}. Remaining books: {}.",
        checked_out_book.title,
        checked_out_book.available,
        library.len()
    );
    // Library now has 2 books "Pride and Prejudice",  "The Hitchhiker's Guide to the Galaxy"
    // The original element in the position 0 "The Lord of the Rings" moved to checked_out_book, and "The Hitchhiker's Guide to the Galaxy" moved from index 1 to 0

    //
    // Method 3: Using std::mem::replace
    //
    // This method gives you the most control, allowing you to swap a specific
    // element with another value you provide. It gives you back the old value
    // and inserts a new one, ensuring the vector remains full and initialized.
    //
    // Assume "Pride and Prejudice" got lost, we replace it with another available copy of it.
    let lost_book = std::mem::replace(
        &mut library[0],
        Book {
            title: "Pride and Prejudice".to_string(),
            available: true,
        },
    );
    println!(
        "Lost book: '{}'. Replaced with a new copy. Remaining books: {}.",
        lost_book.title,
        library.len()
    );
    // Library now has 2 books:  "Pride and Prejudice", "The Hitchhiker's Guide to the Galaxy"
    // The original element in the position 0 moved out to lost_book, which was "Pride and Prejudice" book with availability: false

    //
    // Iterating and consuming all elements.
    //
    // The `for...in` loop moves the entire vector, then provides elements
    // one by one. After this loop, the `library` vector would be uninitialized.
    //
    println!("Remaining Books in the loop:");
    let remaining_books = library;
    for mut book in remaining_books {
        // Note the use of 'mut' to modify the book directly during iteration.
        book.available = false; // Book is no longer available now
        println!("- {} (now checked out)", book.title);
    }
    //remaining_books moved to book, after the loop. The `library` and the "remaining_books" vector are both uninitialized now

    //
    // Using `Option` to handle moves from indexed content more flexibly.
    //
    // Sometimes, you may not always want to move a value out of a struct, but you
    // want the flexibility to do so when needed. `Option<T>` allows for this.
    //

    #[allow(dead_code)]
    struct BorrowableBook {
        title: String,
        borrower: Option<String>, // The borrower's name (if borrowed), otherwise None
    }

    let mut better_library = vec![
        BorrowableBook {
            title: "The Great Gatsby".to_string(),
            borrower: None,
        },
        BorrowableBook {
            title: "Moby Dick".to_string(),
            borrower: Some("Alice".to_string()),
        },
    ];

    // Borrow the Great Gatsby
    let great_gatsby_borrower = better_library[0].borrower.take(); // Take the value, leaving None
    println!("Initial Borrower of 'The Great Gatsby': {:?}", great_gatsby_borrower);

    better_library[0].borrower = Some("Bob".to_string());

    // View the changes: Alice still has "Moby Dick", "The Great Gatsby" now is with "Bob"
    println!(
        "Current borrowers: {}, {}",
        better_library[0].borrower.as_ref().unwrap_or(&"No one".to_string()),
        better_library[1].borrower.as_ref().unwrap_or(&"No one".to_string())
    );

    // Alice returned the Moby Dick
    let alice_returned_book = better_library[1].borrower.take();
    println!("Moby Dick returned by: {:?}", alice_returned_book);

    println!(
        "Current borrowers after Alice returned: {}, {}",
        better_library[0].borrower.as_ref().unwrap_or(&"No one".to_string()),
        better_library[1].borrower.as_ref().unwrap_or(&"No one".to_string())
    );
}

#[allow(dead_code)]
#[allow(unused_assignments)]
pub(crate) fn copy_types_demo() {
    println!("Demonstrating Copy Types in Rust:");

    // **1. Integer Types (i32, u32, etc.)**

    println!("\n- Integer (Copy) Example:");
    let num1: i32 = 42;
    let num2 = num1; // Copy occurs here, not a move. Both num1 and num2 are independent.

    println!("  num1: {}, num2: {}", num1, num2);

    // Changes to num2 do not affect num1.
    let mut num2_mut = num2;
    num2_mut = 99;
    println!(
        "  After modifying num2, num1: {}, num2_mut: {},  (num2: {})",
        num1, num2_mut, num2
    );

    // **2. Floating-Point Types (f32, f64)**

    println!("\n- Floating-Point (Copy) Example:");
    let float1: f64 = 3.14159;
    let float2 = float1;

    println!("  float1: {}, float2: {}", float1, float2);

    // **3. Character Type (char)**

    println!("\n- Character (Copy) Example:");
    let char1: char = 'A';
    let char2 = char1;

    println!("  char1: {}, char2: {}", char1, char2);

    // **4. Boolean Type (bool)**

    println!("\n- Boolean (Copy) Example:");
    let bool1: bool = true;
    let bool2 = bool1;

    println!("  bool1: {}, bool2: {}", bool1, bool2);

    // **5. Tuples of Copy Types**

    println!("\n- Tuple of Copy Types (Copy) Example:");
    let tuple1: (i32, char, bool) = (10, 'Z', false);
    let tuple2 = tuple1; // The whole tuple is copied.

    println!(
        "  tuple1: ({}, {}, {}), tuple2: ({}, {}, {})",
        tuple1.0, tuple1.1, tuple1.2, tuple2.0, tuple2.1, tuple2.2
    );

    // **6. Fixed-Size Arrays of Copy Types**

    println!("\n- Fixed-Size Array of Copy Types (Copy) Example:");
    let array1: [i32; 3] = [1, 2, 3];
    let array2 = array1; // The whole array is copied.

    println!(
        "  array1: [{}, {}, {}], array2: [{}, {}, {}]",
        array1[0], array1[1], array1[2], array2[0], array2[1], array2[2]
    );

    // **7. Non-Copy Types: Structs (default behavior)**

    println!("\n- Non-Copy Type (Struct) Example (Default Behavior):");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let point1 = Point { x: 10, y: 20 };
    // let point2 = point1; // This would cause a move and render point1 unusable
    //  println!("  point1: {:?}, point2: {:?}", point1, point2);
    println!("  point1: {:?}", point1); // This is OK. As point2 will cause to point1 become unusable, and it is a compiler error

    // **8. Making a Struct Copy-able**

    println!("\n- Copy-able Struct Example:");

    #[derive(Debug, Copy, Clone)] // `Copy` requires `Clone`.
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    let color1 = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    let color2 = color1; // Copy occurs due to `#[derive(Copy, Clone)]`
    println!("  color1: {:?}, color2: {:?}", color1, color2);

    // **9. Non-Copy Struct Due to Non-Copy Field**

    println!("\n- Struct with non-Copy Type field - Can Not Be a Copy Example:");

    //   #[derive(Debug, Copy, Clone)] // This would cause an error since String is not a Copy type.
    #[derive(Debug, Clone)]
    struct Message {
        text: String,
        timestamp: u64,
    }

    let message1 = Message {
        text: "Hello, world!".to_string(),
        timestamp: 1678886400, // Example timestamp
    };
    let message2 = message1.clone(); // A Clone should be implemented since message is not a Copy

    println!(
        "  message1: {{ text: '{}', timestamp: {} }}, message2: {{ text: '{}', timestamp: {} }}",
        message1.text, message1.timestamp, message2.text, message2.timestamp
    );
    println!("message 1 timestamp is: {}", message1.timestamp);
}
