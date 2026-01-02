//! Test that flags_list! macro produces correct error for unsupported field type.

use oottracker_derive::flags_list;

extern crate bitflags;
extern crate byteorder;
extern crate ootr;

// This should fail because f32 is not a supported field type.
// Supported types are: i8, u8, i16, u16, i32, u32, i64, u64
flags_list! {
    pub struct InvalidFlags: [f32; 2] {
        0: {
            FLAG_A = 0x0001,
        },
    }
}

fn main() {}
