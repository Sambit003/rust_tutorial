use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
pub(crate) fn rc_arc_demo() {
    /*
        Rc<T> and Arc<T> (Reference Counted and Atomic Reference Counted Smart Pointers)

        Purpose:
        - Enable shared ownership of a value across multiple parts of a program.
        - Automatically manage the lifetime of the shared value, deallocating it when the last reference is dropped.

        Key Differences:

        - Rc<T>:
            - Single-threaded use only. More efficient because it doesn't need atomic operations for thread safety.
            - Faster because it avoids the overhead of atomic operations.
        - Arc<T>:
            - Thread-safe (can be shared across threads). Uses atomic reference counting, which is slightly slower.
            - Slower due to atomic operations but necessary for safe concurrent access.

        How they work:

        1. Creation: `Rc::new()` or `Arc::new()` creates a new reference-counted pointer that owns a value of type `T` on the heap. The initial reference count is 1.

        2. Cloning:  `clone()` creates a new smart pointer that points to the *same* heap-allocated value. The reference count is incremented. Cloning is cheap; it doesn't copy the underlying `T`.

        3. Dropping: When an `Rc` or `Arc` is dropped, the reference count is decremented. When the count reaches 0, the owned `T` value is deallocated from the heap.

        Immutability:

        - The value inside an `Rc<T>` or `Arc<T>` is immutable by default. This is crucial for safety, especially in multi-threaded environments. Trying to directly mutate the `T` value will result in a compiler error.
        - Use `Cell`, `RefCell`, or `Mutex<T>` inside the `Rc<T>` or `Arc<T>` to enable interior mutability if you *really* need shared mutable state, understanding its consequences and usage patterns correctly to uphold safety guarantees.

        Reference Cycles (Memory Leaks):

        - It's possible to create reference cycles (where two or more `Rc`s or `Arcs` point to each other, preventing them from ever reaching a 0 reference count), which leads to memory leaks.
        - This typically requires creating loops by using unsafe constructs or misusing the internal mutability facilities provided by types like `RefCell` and not commonly happens when we use `Rc`s and `Arc`s to provide immutable access or shared ownership to different variables.
        - Weak pointers (`std::rc::Weak` and `std::sync::Weak`) can be used for certain kinds of relationships to avoid cycles.

        Example:

        use std::rc::Rc;

        let shared_data: Rc<String> = Rc::new("Shared Data".to_string());
        let copy1 = shared_data.clone(); // Ref count becomes 2.
        let copy2 = shared_data.clone(); // Ref count becomes 3.

        // Now `shared_data`, `copy1`, and `copy2` all point to the same string on the heap.

        // When all three `Rc`s go out of scope, the string will finally be deallocated.

        When to Use:

        - Use `Rc` when you need shared ownership in a single-threaded context.
        - Use `Arc` when you need shared ownership across threads. Remember to coordinate access to shared mutable data using mechanisms like `Mutex` to prevent data races if interior mutability is also used with the data wrapped with `Arc`.

    */
    println!("Demonstrating Rc and Arc (Shared Ownership) in Rust:");

    // **1. Rc Example (Single-Threaded)**

    println!("\n- Rc (Single-Threaded) Example:");
    let rc_string: Rc<String> = Rc::new("Shared string (Rc)".to_string());
    println!("  Initial reference count: {}", Rc::strong_count(&rc_string)); // Output: 1

    let rc_string_clone1 = rc_string.clone();
    println!(
        "  Reference count after first clone: {}",
        Rc::strong_count(&rc_string)
    ); // Output: 2
    {
        // Cloning creates a new strong reference. Note that a `weak` reference also can be used
        let rc_string_clone2 = rc_string.clone();
        println!(
            "  Reference count inside block scope: {}",
            Rc::strong_count(&rc_string)
        ); // Output: 3
    } // `rc_string_clone2` is dropped here. Its reference is released.

    println!(
        "  Reference count after block scope: {}",
        Rc::strong_count(&rc_string)
    ); // Output: 2

    println!(
        "  The content of the first clone by the scope 'main': {}",
        rc_string_clone1,
    );

    // **2. Arc Example (Multi-Threaded)**

    println!("\n- Arc (Multi-Threaded) Example:");

    let arc_string: Arc<String> = Arc::new("Shared string (Arc)".to_string());
    println!(
        "  Initial reference count: {}",
        Arc::strong_count(&arc_string)
    ); // Output: 1

    let arc_string_clone = arc_string.clone();

    let handle = thread::spawn(move || {
        println!(
            "  Reference count from another thread: {}",
            Arc::strong_count(&arc_string_clone)
        ); // count value at least 2

        println!("  String from another thread: {}", arc_string_clone);

    });

    handle.join().unwrap();
    // Wait for thread

    println!(
        "  Reference count in main thread after join: {}",
        Arc::strong_count(&arc_string)
    );
    // After thread finish count will become again to 1

    // **3. Demonstrating Immutability**

    println!("\n- Demonstrating Immutability: Uncomment the code to see the error.");
    //  rc_string.push_str(" - modified!"); // Error: cannot borrow data in an `Rc` as mutable
    //
    // Rc provides shared ownership to immutable data
    // Use interior mutability to modify data (like using a Mutex<T> inside the Rc).

    // **4. Illustrating Potential for Reference Cycles (but difficult to achieve directly)**
    // (See the detailed explanation for nuances.) Cycles will require use of Cell or RefCell
}