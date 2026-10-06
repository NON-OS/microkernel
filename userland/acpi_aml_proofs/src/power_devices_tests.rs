// NONOS Operating System (AGPL-3.0-or-later)
//! The namespace's power devices are recognised by their _HID.

use crate::arch::x86_64::acpi::aml::power_devices::*;
use crate::fixtures::device;

const PNP: [u8; 2] = [0x41, 0xD0];

fn eisa(lo: u8, hi: u8) -> [u8; 4] {
    [PNP[0], PNP[1], lo, hi]
}

fn string_hid_device(name: &[u8; 4], hid: &[u8]) -> alloc::vec::Vec<u8> {
    let mut body = alloc::vec![0x08u8, b'_', b'H', b'I', b'D', 0x0D];
    body.extend_from_slice(hid);
    body.push(0);
    let mut inner = name.to_vec();
    inner.extend_from_slice(&body);
    let mut out = alloc::vec![0x5Bu8, 0x82, (inner.len() + 1) as u8];
    out.extend_from_slice(&inner);
    out
}

#[test]
fn a_laptop_namespace_declares_battery_ac_lid_and_ec() {
    let mut aml = device(b"BAT0", eisa(0x0C, 0x0A), &[]);
    aml.extend(string_hid_device(b"ADP1", b"ACPI0003"));
    aml.extend(device(b"LID0", eisa(0x0C, 0x0D), &[]));
    aml.extend(device(b"EC0_", eisa(0x0C, 0x09), &[]));
    let d = classify(&aml);
    assert!(d.battery && d.ac_adapter && d.lid && d.embedded_controller);
    assert!(!d.power_button);
}

#[test]
fn a_desktop_namespace_declares_no_battery() {
    let aml = device(b"PWRB", eisa(0x0C, 0x0C), &[]);
    let d = classify(&aml);
    assert!(!d.battery && !d.ac_adapter && !d.lid);
    assert!(d.power_button);
}

#[test]
fn devices_in_an_ssdt_count() {
    let dsdt = device(b"EC0_", eisa(0x0C, 0x09), &[]);
    let ssdt = device(b"BAT1", eisa(0x0C, 0x0A), &[]);
    let d = classify_blocks([dsdt.as_slice(), ssdt.as_slice()]);
    assert!(d.battery && d.embedded_controller);
}
