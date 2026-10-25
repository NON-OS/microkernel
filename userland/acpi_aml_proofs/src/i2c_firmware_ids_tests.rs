// NONOS Operating System (AGPL-3.0-or-later)
//! Firmware devices that are not HID over I2C stay out of the enumeration
//! whatever vendor their `_HID` names. QEMU's tpm-crb declares the TPM 2.0 as
//! `_HID "MSFT0101"` with a Memory32Fixed window and no I2C bus (TCG ACPI
//! Specification, the TPM 2.0 device): it was published as a touchpad.

use alloc::vec::Vec;

use crate::arch::x86_64::acpi::devices::i2c::parse_hid_devices;
use crate::fixtures::{crs_buffer, i2c_serial_bus, memory32_fixed};

/// `Device (seg) { Name (_HID, "hid") Name (_CRS, crs) }`, `_HID` a string.
fn string_device(seg: &[u8; 4], hid: &str, crs: &[u8]) -> Vec<u8> {
    let mut inner = seg.to_vec();
    inner.extend_from_slice(&[0x08, b'_', b'H', b'I', b'D', 0x0D]);
    inner.extend_from_slice(hid.as_bytes());
    inner.push(0);
    inner.extend_from_slice(&[0x08, b'_', b'C', b'R', b'S']);
    inner.extend_from_slice(crs);
    let mut out = alloc::vec![0x5Bu8, 0x82, (inner.len() + 1) as u8];
    out.extend_from_slice(&inner);
    out
}

#[test]
fn qemus_tpm_is_not_a_touchpad() {
    let crs = crs_buffer(&memory32_fixed(0xFED4_0000, 0x5000));
    assert!(parse_hid_devices(&string_device(b"TPM_", "MSFT0101", &crs)).is_empty());
}

#[test]
fn no_other_microsoft_firmware_id_is_taken_for_one() {
    let crs = crs_buffer(&i2c_serial_bus(0x2C));
    for hid in ["MSFT0101", "MSFT0200", "MSFT0002", "MSFT8000"] {
        let found = parse_hid_devices(&string_device(b"DEV0", hid, &crs));
        assert!(found.is_empty(), "{hid} read as a HID-over-I2C device");
    }
}

#[test]
fn microsofts_touchpad_and_the_vendors_still_are_without_a_cid() {
    let crs = crs_buffer(&i2c_serial_bus(0x2C));
    for hid in ["MSFT0001", "ELAN0700", "SYNA3700", "ALPS0002", "CYAP0003", "FTE10001"] {
        let found = parse_hid_devices(&string_device(b"TPD0", hid, &crs));
        assert_eq!(found.len(), 1, "{hid} lost");
        assert_eq!(found[0].slave_addr, 0x2C);
    }
}
