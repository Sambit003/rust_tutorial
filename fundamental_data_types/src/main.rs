// This program is spanned through number of functions to demonstrate the fundamental data types in Rust.
// By default, all the fundamental data types are immutable in Rust, making it a safe lower level language to work with. In this file, we will mostly work with the immutable
// data types and later we will see how to make them mutable.

/*
    Undertanding Type Literal:
    1. What is it? - A type literal is a sequence of characters that represents a type.
    2. Why do we need it? - Type literals are used to specify the type of a variable or a function parameter.
    3. How to use it? - Type literals are used in Rust by appending a colon and the type literal to the variable or function parameter or appending the value with the type.
    4. Example: let x: i32 = 5; // Here, i32 is the type literal.
       Another example is let x = 5; // Here, Rust will infer the type of x as i32.
       Another advanced example is let x = 5i32; // Here, Rust will infer the type of x as i32 and value is 5.
*/

use regex::Regex;

struct ArrayDemo;
struct StringDemo;
struct TupleDemo;
struct TypeLiteralDemo;
struct VectorDemo;

impl TypeLiteralDemo {
    pub fn type_literal_demo() {
        // Type Literal
        let x: i32 = 5; // Here, i32 is the type literal.
        let y = 5; // Here, Rust will infer the type of x as i32.
        let z = 5i32; // Here, Rust will infer the type of x as i32 and value is 5.
        println!("x: {}", x);
        println!("y: {}", y);
        println!("z: {}", z);

        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        } // return type literal
        add(x, y);
    }
}

impl StringDemo {
    pub fn string_demo() {
        // String Literals are of two types: &str and String 
        // in C/C++ it is const char* and in C++, as std::string class offering dynamic creation at runtime
        // In Rust, &str is a string slice, a reference to a UTF-8 encoded string in memory, and String is a heap-allocated string, a growable, mutable, owned UTF-8 encoded string.

        // Char Literal
        let char_literal = 'a';
        print!("Char Literal: {}\n", char_literal);
        let single_quote_char_literal = '\'';
        print!("Single Quote Char Literal: {}\n", single_quote_char_literal);

        // String Literal
        let string_literal = "\"Hiii\"Hello' World!\n";
        print!("String Literal: {}\n", string_literal);
        // In string lateral, unlike char literal, single qoutes don't need to be escaped, but double quotes need to be escaped.

        // Multiline String Literal
        println!("Multiline String Literal:
                    Line 1
                    Line 2");  // Here newline and spaces are preserved
        println!("Multiline String Literal: \
                    Not in Line 1, \
                    Not in Line 2"); // Here newline and spaces are not preserved
        
        // Raw string literal: Here no escape sequence is processed
        let this_file_path = r"./home/usr/this_file.txt";
        println!("Raw String Literal: {}", this_file_path);
        let regex_raw_string = Regex::new(r"\d{4}-\d{2}-\d{2}");
        println!("Raw String Literal: {:?}", regex_raw_string);
        // As we can't include a double quote in a raw string literal, we can use # as delimiter
        println!(r###"
            This raw string started with 'r###"'.
            Therefore it does not end until we reach a quote mark ('"')
            followed immediately by three pound signs ('###'):
        "###);

        // Byte String: In Rust, a byte string is a sequence of bytes, represented by the type [u8; N] or its borrowed form &[u8].
        let byte_string = b"Hello, World!";
        println!("Byte String: {:?}", byte_string);
        assert_eq!(byte_string, &[72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]);
        /*
            Raw Byte strings starts with br"
            Byte string can't contain Unicode Characters, but it can be enabled with ASCII encoding and \xHH escape sequence
         */
        // Example of Unicode Characters in Byte String
        let byte_string_unicode = br"\x48\x65\x6c\x6c\x6f\x2c\x20\xe4\xb8\x96\xe7\x95\x8c";
        println!("Byte String with Unicode Characters values: {:?}", byte_string_unicode);

        
    }
}

impl ArrayDemo {
    pub fn array_demo() {
        // Simplest way of decalaring the array
        let array: [u32; 6] = [1, 2, 3, 4, 5, 6]; // Array of maximum 6 elements of type unsigned integer 32 bits (u32)
        let explicit_string_array: [&str; 3] = ["Hello", "World", "!"]; // Array of maximum 3 elements of type string slice with explicit type declaration
        let implicit_string_array = ["Hello", "World"]; // Array of maximum 2 elements of type string slice with implicit type declaration [rust analyzer will show you the type and element count]
        assert_eq!(array[0], 1);
        assert_eq!(array.len(), 6);
        assert_eq!(explicit_string_array.len(), 3);
        assert_eq!(implicit_string_array.len(), 2);
        println!("Array: {:?}", array);
        println!("String Array: {:?}", explicit_string_array);
        println!("Normal String Array: {:?}", implicit_string_array);

        /* For the common case of a long array filled with some value, you can write
        [V; N], where V is the value each element should have, and N is the
        length. For example, [true; 10000] is an array of 10,000 bool
        elements, all set to true: */
        let mut sieve = [true; 10000];
        for i in 2..100 {
            if sieve[i] {
                let mut j = i * i;
                while j < 10000 {
                    sieve[j] = false;
                    j += i;
                }
            }
        }
        assert!(sieve[211]);
        assert!(!sieve[9876]);

        // Array of 100 elements with default value 0
        let mut array_100 = [0; 100]; // Array of 100 elements with default value 0 (mutable array)
                                      /* Why Mutable Array?
                                       * 1. You can change the value of the array elements
                                       * 2. You can change the length of the array
                                       */
        println!("Array_100 before changing each values: {:?}", array_100);
        // Changing the value of the array elements
        for i in 0..100 {
            array_100[i] = i as i32;
        }
        println!("Array_100 after changing each values: {:?}", array_100);

        //Let's have a fixed size buffer array
        let buffer1 = [0u8; 1024]; // Method 1
        let buffer2: [u8; 1024] = [0; 1024]; // Method 2
        println!("Buffer1: {:?}", buffer1);
        println!("Buffer2: {:?}", buffer2);
    }
}

impl TupleDemo {
    pub fn tuple_demo() {
        /*  A tuple is a pair, or triple, quadruple, quintuple, etc. (hence, n-tuple, or
        tuple), of values of assorted types. You can write a tuple as a sequence of
        elements, separated by commas and surrounded by parentheses. For
        example, ("Brazil", 1985) is a tuple whose first element is a
        statically allocated string, and whose second is an integer; its type is
        (&str, i32). Given a tuple value t, you can access its elements as
        t.0, t.1, and so on. */
        // Basic Tuple
        let tuple = (1, 2, 3, 4, 5);
        println!("Tuple: {:?}", tuple);
        println!("Tuple.0: {}", tuple.0);
        println!("Tuple.1: {}", tuple.1);
        println!("Tuple.2: {}", tuple.2);
        println!("Tuple.3: {}", tuple.3);
        println!("Tuple.4: {}", tuple.4);

        // Tuple demo with string destructuring
        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        assert_eq!(head, "I see the eigenvalue ");
        assert_eq!(tail, "in thine eye");

        // Tuple allocation in Heap
        let t = (12, "eggs");
        let b = Box::new(t);
        println!("b: {:?}", b);
        // memory Address of the heap allocated tuple
        let address = &b as *const _ as usize;
        println!("Address of b: {:x}", address);
    }
}

impl VectorDemo {
    // A vector Vec<T> is a resizable array of elements of type T, allocated on the heap.
    pub fn vector_demo() {
        /*
        There are several ways to create vectors. The simplest is to use the vec!
        macro, which gives us a syntax for vectors that looks very much like an
        array literal:
                 */
        let mut prime_vec = vec![2, 3, 5, 7];
        assert_eq!(prime_vec.iter().product::<i32>(), 210);

        // Vector is not array, so we can add elements to it dynamically
        prime_vec.push(11);
        prime_vec.push(13);
        assert_eq!(prime_vec.iter().product::<i32>(), 30030);

        //vec! is a macro, and also equivalent to Vec::new() method but you can't initialise it at the time of variable declaration
        // Heap will not be allocated until you push the elements to the vector
        let mut v: Vec<i32> = Vec::new();
        v.push(10);
        v.push(20);
        v.push(30);
        assert_eq!(v, vec![10, 20, 30]);

        // Vector declaration with iterator
        let v: Vec<i32> = (0..5).collect();
        assert_eq!(v, [0, 1, 2, 3, 4]);

        // A palindrome!
        let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
        palindrome.reverse();
        // Reasonable yet disappointing:
        assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);
        /*
        Why Disappointing?
            - The reverse method is defined on slices, not vectors. When you call reverse on a vector, the compiler implicitly borrows a slice from the vector and calls reverse on that slice.
        Implicit Borrowing and Slicing:
            - In Rust, when you call a method on a vector, the compiler implicitly borrows a slice from the vector and invokes the method on that slice. This is done to avoid unnecessary copying of the entire vector.
            - In this case, the reverse method is defined on slices, so the compiler borrows a mutable slice &mut [&str] from the palindrome vector and calls reverse on that slice.

        Explanation of the Assertion:
            - The assertion assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]); checks if the reversed vector is equal to the original vector in reverse order.
            - Since the reverse method is called on a slice of the vector, the original vector itself is not modified. Instead, the slice is reversed in-place.
            - Therefore, the assertion passes because the slice is reversed, but the original vector remains unchanged.
        */

        // Now let's see how to create a vector with limited capacity
        let mut vec_limitted= Vec::with_capacity(2);
        assert_eq!(vec_limitted.len(), 0);
        assert_eq!(vec_limitted.capacity(), 2);
        // Now pushing the elements to the vector
        vec_limitted.push(1);
        vec_limitted.push(2);
        assert_eq!(vec_limitted.len(), 2);
        assert_eq!(vec_limitted.capacity(), 2);

        // Let's try to push one more element to the vector
        vec_limitted.push(3);
        assert_eq!(vec_limitted.len(), 3);
        assert_eq!(vec_limitted.capacity(), 4); 
        // Why 4? because the heap is allocated in chunks of 2x the current capacity when the capacity is reached and the vector needs to grow.
        // Let's print the vector
        println!("Vector with limited capacity: {:?}", vec_limitted);
    }
}

fn main() {
    print!("Type Literal Demo: \n");
    TypeLiteralDemo::type_literal_demo();

    print!("String Demo: \n");
    StringDemo::string_demo();

    print!("Array Demo: \n");
    ArrayDemo::array_demo();

    print!("Tuple Demo: \n");
    TupleDemo::tuple_demo();

    print!("Vector Demo: \n");
    VectorDemo::vector_demo();
}
