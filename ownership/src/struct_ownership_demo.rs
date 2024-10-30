pub(crate) fn struct_ownership_demo() {
    #[derive(Debug)]
    struct Fruit {
        name: String,
        color: String,
    }

    let mut fruit_vec = Vec::new();
    fruit_vec.push(Fruit {
        name: "Apple".to_string(),
        color: "Red".to_string(),
    });
    fruit_vec.push(Fruit {
        name: "Banana".to_string(),
        color: "Yellow".to_string(),
    });
    fruit_vec.push(Fruit {
        name: "Grapes".to_string(),
        color: "Green".to_string(),
    });

    for fruit in fruit_vec.iter() {
        println!("Fruit: {}, Color: {}", fruit.name, fruit.color);
    }

    // Let's dive into the memory stuffs
    println!("\n\n::STRUCT OWNERSHIP IN ACTION::");
    println!("Fruit Vector buffer address: {:x} , value: {:?}\nFruit Vector capacity address: {:x} , value: {}\nFruit Vector length address: {:x} , value: {}",
             &fruit_vec as *const _ as usize, fruit_vec,
             &fruit_vec.capacity() as *const _ as usize, fruit_vec.capacity(),
             &fruit_vec.len() as *const _ as usize, fruit_vec.len());
    for (i, fruit) in fruit_vec.iter().enumerate() {
        println!("Fruit at buffer index {}: {:?}", i, fruit);
        // Address of the Fruit struct members and respective heap
        let name_address = &fruit.name as *const _ as usize;
        let color_address = &fruit.color as *const _ as usize;
        println!("Fruit name address: {:x}, value: {}\nFruit Name Capacity Address: {:x}, value: {}\nFruit name length address: {:x}, value: {}\n",
                 name_address, fruit.name,
                 &fruit.name.capacity() as *const _ as usize, fruit.name.capacity(),
                 &fruit.name.len() as *const _ as usize, fruit.name.len());
        println!("Fruit color address: {:x}, value: {}\nFruit Color Capacity Address: {:x}, value: {}\nFruit color length address: {:x}, value: {}\n",
                    color_address, fruit.color,
                    &fruit.color.capacity() as *const _ as usize, fruit.color.capacity(),
                    &fruit.color.len() as *const _ as usize, fruit.color.len());


    }
}
