// NONOS Operating System (AGPL-3.0-or-later)
//! Bring-up in ax88179_bind and ax88179_reset's order, with the bytes
//! Linux writes (the expected values are spelled out from
//! ax88179_178a.c, not taken from the driver's constants).

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Nic, Setup};

use crate::bound::bound;
use crate::calls::{mac_r, mac_w, phy_w};
use crate::chip::{Chip, BMCR, MAC};

#[test]
fn reset_runs_in_linux_order_with_linux_values_and_offload_off() {
    let before = nonos_libc::slept_ms();
    let (bus, nic) = bound(&Chip::default());
    let calls = bus.0.borrow().calls.clone();
    assert!(matches!(calls[0], Call::Configure(p) if p.bulk_in == 0x82 && p.bulk_out == 0x03));
    let expected = vec![
        Call::Out(Setup::set_configuration(1), vec![]),
        Call::Out(Setup::set_interface(0, 0), vec![]),
        mac_w(0x26, &[0x00, 0x00]),
        mac_w(0x26, &[0x20, 0x00]),
        mac_w(0x33, &[0x03]),
        Call::In(Setup::new(0xC0, 0x04, 0x43, 1), 2),
        mac_r(0x10, 6),
        mac_w(0x10, &MAC),
        mac_w(0x2e, &[7, 0x4f, 0, 2, 0xff]),
        mac_w(0x55, &[0x34]),
        mac_w(0x54, &[0x52]),
        mac_w(0x34, &[0]),
        mac_w(0x35, &[0]),
        mac_w(0x0b, &[0xaa, 0x03]),
        mac_w(0x24, &[0x64]),
        mac_w(0x22, &[0x33, 0x01]),
        phy_w(0x1f, 3),
        phy_w(0x19, 0x3246),
        phy_w(0x1f, 0),
        phy_w(0x0d, 7),
        phy_w(0x0e, 60),
        phy_w(0x0d, 0x4007),
        phy_w(0x0e, 0),
        Call::In(Setup::new(0xC0, 0x02, 3, 0), 2),
        phy_w(0x00, BMCR | 0x0200),
    ];
    assert_eq!(calls[1..], expected[..]);
    assert_eq!(nic.mac(), MAC);
    assert!(!nic.link_up(), "Linux starts with the carrier off");
    assert!(nonos_libc::slept_ms() - before >= 300, "msleep(200) and msleep(100) on the clock");
}

#[test]
fn the_eeprom_asks_for_auto_detach_and_gets_it() {
    let chip = Chip { eeprom: Some(0x0100), ..Chip::default() };
    let (bus, _) = bound(&chip);
    let calls = bus.0.borrow().calls.clone();
    let at = calls.iter().position(|c| *c == Call::In(Setup::new(0xC0, 4, 0x43, 1), 2));
    let at = at.expect("EEPROM read") + 1;
    let detach = [mac_r(0x33, 1), mac_w(0x33, &[0x0b]), mac_r(0x26, 2), mac_w(0x26, &[0x20, 0x10])];
    assert_eq!(calls[at..at + 4], detach[..]);
    assert_eq!(calls[at + 4], mac_r(0x10, 6));
}

#[test]
fn an_erased_or_clear_eeprom_word_leaves_auto_detach_off() {
    for word in [0xffff, 0x0000, 0x00ff] {
        let (bus, _) = bound(&Chip { eeprom: Some(word), ..Chip::default() });
        let calls = bus.0.borrow().calls.clone();
        let at = calls.iter().position(|c| *c == Call::In(Setup::new(0xC0, 4, 0x43, 1), 2));
        assert_eq!(calls[at.unwrap() + 1], mac_r(0x10, 6), "word {word:#06x}");
    }
}
