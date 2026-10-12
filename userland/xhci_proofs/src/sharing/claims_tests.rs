// NONOS Operating System (AGPL-3.0-or-later)
//! Two class drivers sharing one device: each holds its own interfaces,
//! the shared slot is found by its port, and the slot is given back only
//! with the last live holder. A device nobody claimed is not shared.

use crate::slots::{Release, Sharing};

const HID: u32 = 10;
const FIDO: u32 = 20;
fn alive(_: u32) -> bool {
    true
}

#[test]
fn an_interface_has_one_holder_and_its_device_one_port() {
    let mut c = Sharing::default().claims;
    assert_eq!(c.claim(4, 3, 0, HID), Ok(()));
    assert_eq!(c.claim(4, 3, 0, HID), Ok(()), "again by its holder");
    assert_eq!(c.claim(4, 3, 0, FIDO), Err(-16));
    assert_eq!(c.claim(4, 3, 1, FIDO), Ok(()));
    assert_eq!(c.claim(4, 5, 2, FIDO), Err(-22), "slot 4 is on port 3");
    assert_eq!(c.claim(0, 3, 2, FIDO), Err(-22));
    assert_eq!(c.on_port(3), Some((4, 0b11)));
    assert_eq!(c.on_port(9), None, "nothing claimed there: not shared");
}

#[test]
fn the_slot_goes_with_its_last_holder() {
    let mut c = Sharing::default().claims;
    c.claim(4, 3, 0, HID).unwrap();
    c.claim(4, 3, 1, FIDO).unwrap();
    assert_eq!(c.release(4, 99, alive), Release::NotHolder, "a stray give-back");
    assert_eq!(c.on_port(3), Some((4, 0b11)), "and it changed nothing");
    assert_eq!(c.release(4, HID, alive), Release::Kept);
    assert_eq!(c.on_port(3), Some((4, 0b10)));
    assert_eq!(c.release(4, FIDO, alive), Release::Last);
    assert_eq!(c.on_port(3), None);
    assert_eq!(c.release(4, FIDO, alive), Release::Unclaimed, "as before sharing");
}

#[test]
fn a_dead_holder_does_not_keep_the_slot() {
    let mut c = Sharing::default().claims;
    c.claim(4, 3, 0, HID).unwrap();
    c.claim(4, 3, 1, FIDO).unwrap();
    assert_eq!(c.release(4, HID, |p| p != FIDO), Release::Last);
    assert_eq!(c.on_port(3), None);
}

#[test]
fn the_table_is_bounded() {
    let mut c = Sharing::default().claims;
    for i in 0..64u8 {
        assert_eq!(c.claim(1 + i / 32, 1 + i / 32, i % 32, HID), Ok(()));
    }
    assert_eq!(c.claim(9, 9, 0, HID), Err(-16), "full");
    assert_eq!(c.on_port(1), Some((1, u32::MAX)));
}
