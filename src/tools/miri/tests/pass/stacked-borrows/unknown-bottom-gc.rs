//@compile-flags: -Zmiri-permissive-provenance
#![feature(strict_provenance)]

use std::ptr;

fn main() {
    let mut v = 1u8;
    let ptr = &mut v as *mut u8;

    // Expose the allocation and use the exposed pointer, creating an unknown bottom
    unsafe {
        let p: *mut u8 = ptr::from_exposed_addr::<u8>(ptr.expose_addr()) as *mut u8;
        *p = 1;
    }

    // Pile on a lot of SharedReadOnly at the top of the stack
    let r = &v;
    for _ in 0..1024 {
        let _x = &*r;
    }
}
