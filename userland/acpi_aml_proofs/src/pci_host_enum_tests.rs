// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that an I2C host declared as a PCI function (LPSS), by its `_ADR`,
//! is found under the name a touchpad's I2cSerialBus names.

use alloc::vec::Vec;

use crate::arch::x86_64::acpi::devices::i2c::{
    parse_hid_devices, parse_pci_controllers, I2cPciHost,
};
use crate::fixtures::{crs_buffer, device};

const PNP0C50: [u8; 4] = [0x41, 0xD0, 0x0C, 0x50];

fn wrap(op: &[u8], content: &[u8]) -> Vec<u8> {
    let n = content.len();
    let len: Vec<u8> = if n + 1 < 64 {
        alloc::vec![(n + 1) as u8]
    } else {
        alloc::vec![0x40 | ((n + 2) & 0xF) as u8, ((n + 2) >> 4) as u8]
    };
    [op, &len, content].concat()
}

/// `Device (seg) { Name (_ADR, adr) }`, the way LPSS functions are declared.
fn pci_device(seg: &[u8; 4], adr: u32) -> Vec<u8> {
    wrap(&[0x5B, 0x82], &[seg, &b"\x08_ADR\x0C"[..], &adr.to_le_bytes()].concat())
}

fn i2c_bus(addr: u16, source: &str) -> Vec<u8> {
    let mut d = alloc::vec![0x02, 0x00, 0x01, 0x02, 0x00, 0x00, 0x01, 0x06, 0x00];
    d.extend_from_slice(&400_000u32.to_le_bytes());
    d.extend_from_slice(&addr.to_le_bytes());
    d.extend_from_slice(source.as_bytes());
    d.push(0);
    [&[0x8E][..], &(d.len() as u16).to_le_bytes(), &d].concat()
}

fn sb(body: &[u8]) -> Vec<u8> {
    wrap(&[0x10], &[&b"\\_SB_"[..], body].concat())
}

#[test]
fn a_touchpad_names_the_lpss_host_whose_adr_is_found() {
    let pad = device(b"TPD0", PNP0C50, &crs_buffer(&i2c_bus(0x2C, "\\_SB.I2C1")));
    let aml = sb(&[pci_device(b"I2C1", 0x0017_0003), pad].concat());
    let pads = parse_hid_devices(&aml);
    assert_eq!(pads.len(), 1);
    assert_eq!(pads[0].controller, *b"I2C1");
    let hosts = parse_pci_controllers(&aml);
    assert_eq!(hosts, alloc::vec![I2cPciHost { name: *b"I2C1", adr: 0x0017_0003 }]);
}

#[test]
fn a_device_without_a_constant_adr_is_not_a_pci_host() {
    let method_adr =
        wrap(&[0x5B, 0x82], &[&b"I2C2"[..], &[0x14, 0x06], b"_ADR", &[0x00, 0xA4, 0x00]].concat());
    let aml = sb(&[method_adr, device(b"PLAT", PNP0C50, &crs_buffer(&[]))].concat());
    assert!(parse_pci_controllers(&aml).is_empty());
}

#[test]
fn the_pci_host_scan_never_panics_on_arbitrary_bytes() {
    let mut seed = 0x2545_F491_4F6C_DD1Du64;
    for len in 0..512usize {
        let v: Vec<u8> = (0..len)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed as u8
            })
            .collect();
        let _ = parse_pci_controllers(&v);
    }
}
