// NONOS Operating System (AGPL-3.0-or-later)
//! A hub descriptor whose length cannot hold DeviceRemovable for the ports
//! it names misnames its port count, and the hub is not taken; the TT think
//! time reaches the controller only from a high-speed hub.

use nonos_usb_hub::parse_hub_descriptor;

#[test]
fn a_length_that_cannot_hold_device_removable_is_refused() {
    // 28 ports need 4 bytes of DeviceRemovable after the 7-byte head; 8 need
    // 2 (QEMU's 10 holds them); a SuperSpeed hub's are its 11th and 12th.
    assert_eq!(parse_hub_descriptor(&[0x0a, 0x29, 0x1c, 0, 0, 0x32, 0], false), None);
    assert_eq!(parse_hub_descriptor(&[0x08, 0x29, 0x08, 0, 0, 0x32, 0], false), None);
    assert!(parse_hub_descriptor(&[0x09, 0x29, 0x08, 0, 0, 0x32, 0], false).is_some());
    assert_eq!(parse_hub_descriptor(&[0x0b, 0x2a, 0x04, 0, 0, 0x32, 0], true), None);
}

#[test]
fn only_a_high_speed_hub_passes_its_think_time_on() {
    use crate::fake::{root_hub, Fake, HubModel};
    use nonos_usb_hub::{SPEED_FULL, SPEED_HIGH};
    let mut h = Fake::default();
    root_hub(&mut h, HubModel::new(false, 4), 2, SPEED_FULL);
    assert_eq!(h.configured, vec![(1, 4, 0)], "a full speed hub has no TT");
    let mut h = Fake::default();
    root_hub(&mut h, HubModel::new(false, 4), 2, SPEED_HIGH);
    assert_eq!(h.configured, vec![(1, 4, 1)]);
}
