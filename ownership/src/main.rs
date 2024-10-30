mod value_ownership_demo;
mod box_generic_type_demo;
mod struct_ownership_demo;

fn main() {
    // value ownsership demo
    println!("--------STARTING OF VALUE OWNERSHIP DEMO--------");
    value_ownership_demo::print_value();
    value_ownership_demo::access_static_address();
    println!("--------END OF VALUE OWNERSHIP DEMO--------\n");

    // Box<T> type ownership demo
    println!("--------STARTING OF BOX<T> TYPE OWNERSHIP DEMO--------");
    box_generic_type_demo::box_demo();
    println!("--------END OF BOX<T> TYPE OWNERSHIP DEMO--------");

    // Struct ownership demo
    println!("--------STARTING OF STRUCT OWNERSHIP DEMO--------");
    struct_ownership_demo::struct_ownership_demo();
    println!("--------END OF STRUCT OWNERSHIP DEMO--------");
}
