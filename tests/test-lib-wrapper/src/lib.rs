use std::os::raw::{c_char, c_int};


// when you compile a binary, the compiler moves stuff around and renames things to optimize it
// no mange tells the compiler not to do that so we can directly interact with the function
#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}


// 0 = success 1 = failure
#[no_mangle]
pub extern "C" fn print_two_statements(a: *mut c_char, b: *mut c_char) -> c_int {
    //  TODO => translate the c_char pointers to rust strings (should have two match statements per pointer)
    // check if the pointers are null => return 1 if null
    // convert to rust strings if not null => return 1 if conversion fails
    println!("First statement: {:?}", a);
    println!("Second statement: {:?}", b);
    0
}
