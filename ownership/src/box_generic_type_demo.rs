// In this file we will play with Box<T> type to demonstrate how ownership works in Rust.
// This Box<T> type is a smart pointer that owns the heap-allocated memory and is responsible for cleaning up the memory when the Box<T> goes out of scope.
// Even vector allocation is based on Box<T> type. This is the root.

pub(crate) fn box_demo() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);

    // Address and values of the point variable attributes
    println!("Point variable address: {:x}, value: {:?}\n", &point as *const _ as usize, point);
    // Heap elements and address of point variable
    println!("point variable Heap elements:");
    let addresses = [
        &point.0 as *const _ as usize,
        &point.1 as *const _ as usize,
    ]; // Box<T> is a pointer, so accessing its attributes requires dereferencing with dot (.) operator
    let values = [point.0, point.1];
    for i in 0..2 {
        println!("Value at index {}: {}, Address: {:x}", i, values[i], addresses[i]);
    }

    // Address and values of the label variable attributes
    println!("\nLabel variable buffer address: {:x}, value: {}", &label as *const _ as usize, label);
    println!("Label variable capacity address: {:x}, value: {}", &label.capacity() as *const _ as usize, label.capacity());
    println!("Label variable length address: {:x}, value: {}", &label.len() as *const _ as usize, label.len());
    // Heap elements and address of label variable
    println!("label variable Heap (Buffer) elements:");
    for (i, byte) in label.as_bytes().iter().enumerate() {
        let address = byte as *const _ as usize;
        println!("Value at buffer index {}: {}, Address: {:x}", i, *byte as char, address);
    }
}