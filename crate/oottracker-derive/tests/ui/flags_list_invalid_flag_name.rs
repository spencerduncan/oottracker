//! Test that flags_list! macro produces error for invalid flag name syntax.
//!
//! This test verifies that using a number as a flag name produces a parse error.

use oottracker_derive::flags_list;

extern crate bitflags;
extern crate byteorder;
extern crate ootr;

// This should fail because 123 is not a valid flag name (must be ident, string, or event)
flags_list! {
    pub struct InvalidFlags: [u16; 2] {
        0: {
            123 = 0x0001,
        },
    }
}

fn main() {}
