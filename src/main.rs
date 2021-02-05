use std::{mem, ptr};

fn main() {
    let mut start_of_stack = 15u8;
    let mut sos_ptr: *mut u8 = &mut start_of_stack;

    unsafe { ptr::write_volatile(sos_ptr, 69u8) };
    let y = unsafe { ptr::read_volatile(&start_of_stack) };
    println!("y is {:?}", y);
    // printing y outputs 69, which we wrote to that address
    //////////////////////////////////////////////////////////////////////
    {
        // artificial scope here
        let foo = 1u8;
        let bar = 1u8;
        let baz = 1u32;
        let quux = 1u8;

        let foo_ptr: *const _ = &foo;
        let bar_ptr: *const _ = &bar;
        let baz_ptr: *const _ = &baz;
        let quux_ptr: *const _ = &quux;
        // the fact that the pointer only increments by 1 suggests 8 bits may be stored at each address?
        println!("foo_ptr = {:?}", foo_ptr); // 1 space between this ptr and next
        println!("bar_ptr = {:?}", bar_ptr); // 1 space between this ptr and next
        println!("baz_ptr = {:?}", baz_ptr); // 7 spaces between this pointer and next
        println!("quux_ptr ={:?}", quux_ptr);

        println!(
            "diff between baz_ptr and quux_ptr is {}",
            quux_ptr as u8 - baz_ptr as u8
        );
        println!("byte literal {:b}", foo_ptr as u64);
    }
}
