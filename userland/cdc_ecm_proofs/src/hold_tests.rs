// NONOS Operating System (AGPL-3.0-or-later)
//! Every interface of the configuration held before it is set; one another
//! driver holds stops the bind before the device is touched.

use nonos_usbnet::mock::{drawn, Call};
use nonos_usbnet::qemu_usb_net::MAC_STRING;

use crate::device::{bound, device, ASKED, HELD};
use crate::ecm::bind;

#[test]
fn both_interfaces_are_held_before_the_configuration_is_set() {
    ASKED.with(|a| a.borrow_mut().clear());
    assert!(bound(|_| {}, &MAC_STRING).is_ok());
    assert_eq!(ASKED.with(|a| a.borrow().clone()), [(2, 0), (2, 1)]);
}

#[test]
fn an_interface_another_driver_holds_is_never_taken_over() {
    for held in [0u8, 1] {
        HELD.with(|h| *h.borrow_mut() = vec![held]);
        let (bus, found) = device(|_| {}, &MAC_STRING);
        let r = bind(bus.clone(), &found, drawn());
        assert!(matches!(r, nonos_usbnet::Bind::Failed("interface held by another driver", -16)));
        let calls = bus.0.borrow().calls.clone();
        assert!(!calls.iter().any(|c| matches!(c, Call::Out(..) | Call::Configure(_))));
    }
    HELD.with(|h| h.borrow_mut().clear());
}
