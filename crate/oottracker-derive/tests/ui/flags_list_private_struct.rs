//! Test that flags_list! macro works correctly with private (non-pub) structs.
//!
//! This is a passing test that verifies private structs are handled correctly.

use oottracker_derive::flags_list;

extern crate bitflags;
extern crate byteorder;
extern crate ootr;

// Private struct should work correctly
flags_list! {
    struct PrivateFlags: [u32; 2] {
        0: {
            FLAG_A = 0x0000_0001,
            FLAG_B = 0x0000_0002,
        },
        1: {
            FLAG_C = 0x0000_0001,
        },
    }
}

fn main() {
    // Test that the private struct can be default-constructed
    let flags = PrivateFlags::default();

    // Test that TryFrom<Vec<u8>> works
    let data: Vec<u8> = vec![0; 8]; // 2 fields * 4 bytes each
    let result = PrivateFlags::try_from(data);
    assert!(result.is_ok());

    // Test serialization
    let bytes: Vec<u8> = (&flags).into();
    assert_eq!(bytes.len(), 8);

    // Test that individual field types are generated
    let _field0 = PrivateFlags0::FLAG_A;
    let _field1 = PrivateFlags1::FLAG_C;
}
