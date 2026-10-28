// NONOS Operating System (AGPL-3.0-or-later)
//! Which devices are taken: the listed ones with the chip's vendor
//! interface; an unlisted one, or a listed one offering only a class
//! configuration, is another driver's and is never touched.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Setup;

use crate::chip::{CONFIG, DEVICE};
use crate::harness::HELD;
use crate::seen::verdict;

#[test]
fn a_listed_device_without_the_vendor_interface_is_left_alone() {
    let class = |at: usize, v: u8| {
        let mut c = CONFIG.to_vec();
        c[at] = v;
        c
    };
    // A communications interface, a vendor one of another subclass or
    // protocol, and one at alternate 1.
    for config in [class(14, 0x02), class(15, 0x06), class(16, 0x01), class(12, 1)] {
        assert_eq!(verdict(DEVICE, vec![config]), ("not ours", vec![]));
    }
    let mut no_bulk_out = CONFIG.to_vec();
    no_bulk_out[47] = 0x03; // the OUT endpoint made an interrupt one
    assert_eq!(verdict(DEVICE, vec![no_bulk_out]).0, "not ours");
}

#[test]
fn the_vendor_configuration_is_set_when_it_is_not_the_first() {
    let mut cdc = CONFIG.to_vec();
    cdc[14] = 0x02;
    let mut second = CONFIG.to_vec();
    second[5] = 2;
    let mut two = DEVICE;
    two[17] = 2;
    let (said, calls) = verdict(two, vec![cdc, second]);
    assert_eq!(said, "ours");
    assert_eq!(calls[1], Call::Out(Setup::set_configuration(2), vec![]));
}

#[test]
fn an_interface_another_driver_holds_is_never_taken_over() {
    HELD.with(|h| *h.borrow_mut() = vec![0]);
    let (said, calls) = verdict(DEVICE, vec![CONFIG.to_vec()]);
    HELD.with(|h| h.borrow_mut().clear());
    assert_eq!(said, "interface held by another driver");
    assert!(!calls.iter().any(|c| matches!(c, Call::Out(..) | Call::Configure(_))));
}
