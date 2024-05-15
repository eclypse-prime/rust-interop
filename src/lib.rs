#[no_mangle]
pub extern "C" fn rs_print() {
    println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn rs_add(a: i32, b: i32) -> i32
{
    a + b
}