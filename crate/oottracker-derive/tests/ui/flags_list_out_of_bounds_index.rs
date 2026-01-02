//! Test that flags_list! macro panics at compile time for out-of-bounds field index.
//!
//! This test verifies that using a field index that exceeds the declared
//! number of fields produces a compile-time error.

use oottracker_derive::flags_list;

extern crate bitflags;
extern crate byteorder;
extern crate ootr;

// This should fail because index 5 exceeds the declared array size of 2
flags_list! {
    pub struct InvalidFlags: [u16; 2] {
        5: {
            FLAG_A = 0x0001,
        },
    }
}

fn main() {}
