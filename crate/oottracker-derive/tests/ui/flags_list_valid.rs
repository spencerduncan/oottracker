//! Test that flags_list! macro generates correct TryFrom implementation
//! and bitflags types for valid input.

use oottracker_derive::flags_list;

// The flags_list! macro generates code that depends on these external crates
// and the ootr crate types for the `checked` method.
extern crate bitflags;
extern crate byteorder;
extern crate ootr;

flags_list! {
    pub struct TestFlags: [u16; 3] {
        0: {
            FLAG_A = 0x0001,
            FLAG_B = 0x0002,
            event "Test Event" = 0x0004,
            "Test Location" = 0x0008,
        },
        1: {
            FLAG_C = 0x0010,
            FLAG_D = 0x0020,
        },
        // Index 2 has no defined flags (tests the default/empty case)
    }
}

fn main() {
    // Test that the struct can be default-constructed
    let flags = TestFlags::default();

    // Test that TryFrom<Vec<u8>> is implemented
    let data: Vec<u8> = vec![0; 6]; // 3 fields * 2 bytes each
    let result = TestFlags::try_from(data);
    assert!(result.is_ok());

    // Test that From<&TestFlags> for Vec<u8> is implemented
    let bytes: Vec<u8> = (&flags).into();
    assert_eq!(bytes.len(), 6);

    // Test that the individual flag types are generated
    let _field0 = TestFlags0::FLAG_A;
    let _field1 = TestFlags1::FLAG_C;
}
