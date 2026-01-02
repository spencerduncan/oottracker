//! Test that flags_list! macro produces error when flag value is missing.
//!
//! This test verifies that omitting the flag value produces a parse error.

use oottracker_derive::flags_list;

extern crate bitflags;
extern crate byteorder;
extern crate ootr;

// This should fail because FLAG_A is missing its value
flags_list! {
    pub struct InvalidFlags: [u16; 2] {
        0: {
            FLAG_A,
        },
    }
}

fn main() {}
