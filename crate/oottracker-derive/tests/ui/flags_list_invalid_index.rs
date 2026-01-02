//! Test that flags_list! macro produces error for non-integer field index.
//!
//! This test verifies that using a non-numeric index produces a parse error.

use oottracker_derive::flags_list;

extern crate bitflags;
extern crate byteorder;
extern crate ootr;

// This should fail because "abc" cannot be parsed as a field index
flags_list! {
    pub struct InvalidFlags: [u16; 2] {
        abc: {
            FLAG_A = 0x0001,
        },
    }
}

fn main() {}
