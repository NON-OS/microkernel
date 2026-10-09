// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the HID-over-I2C enumeration (src/arch/x86_64/acpi/devices/i2c)
//! against AML laid out the way laptop firmware writes it: a vendor `_HID`
//! with the PNP0C50 compatible id, Intel reference code's "XXXX0000"
//! placeholder, a `_DSM` naming the descriptor register, GpioInt and APIC
//! interrupts, and an AMD platform controller.

use alloc::vec::Vec;

use crate::arch::x86_64::acpi::devices::i2c::{
    parse_hid_devices, parse_platform_controllers, HidInterrupt, I2cHidDeviceType,
};

const PNP0C50: [u8; 4] = [0x41, 0xD0, 0x0C, 0x50];
const GUID: [u8; 16] = [
    0xF7, 0xF6, 0xDF, 0x3C, 0x67, 0x42, 0x55, 0x45, 0xAD, 0x05, 0xB3, 0x0A, 0x3D, 0x89, 0x38, 0xDE,
];

fn pkg(content: &[u8]) -> Vec<u8> {
    let n = content.len();
    let mut out = if n + 1 < 64 {
        alloc::vec![(n + 1) as u8]
    } else {
        let t = n + 2;
        assert!(t < 4096);
        alloc::vec![0x40 | (t & 0xF) as u8, (t >> 4) as u8]
    };
    out.extend_from_slice(content);
    out
}

fn cat(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}

fn name(seg: &[u8; 4], value: &[u8]) -> Vec<u8> {
    cat(&[&[0x08], seg, value])
}

fn string(s: &str) -> Vec<u8> {
    cat(&[&[0x0D], s.as_bytes(), &[0]])
}

fn eisa(id: [u8; 4]) -> Vec<u8> {
    cat(&[&[0x0C], &id])
}

fn buffer(bytes: &[u8]) -> Vec<u8> {
    let inner = cat(&[&[0x0A, bytes.len() as u8], bytes]);
    cat(&[&[0x11], &pkg(&inner)])
}

fn device(seg: &[u8; 4], body: &[u8]) -> Vec<u8> {
    cat(&[&[0x5B, 0x82], &pkg(&cat(&[seg, body]))])
}

fn scope(path: &[u8], body: &[u8]) -> Vec<u8> {
    cat(&[&[0x10], &pkg(&cat(&[path, body]))])
}

fn method(seg: &[u8; 4], flags: u8, body: &[u8]) -> Vec<u8> {
    cat(&[&[0x14], &pkg(&cat(&[seg, &[flags], body]))])
}

fn if_(pred: &[u8], body: &[u8]) -> Vec<u8> {
    cat(&[&[0xA0], &pkg(&cat(&[pred, body]))])
}

fn large(lead: u8, data: &[u8]) -> Vec<u8> {
    cat(&[&[lead], &(data.len() as u16).to_le_bytes(), data])
}

fn i2c_bus(addr: u16, speed: u32, source: &str) -> Vec<u8> {
    let mut d = alloc::vec![0x02, 0x00, 0x01, 0x02, 0x00, 0x00, 0x01, 0x06, 0x00];
    d.extend_from_slice(&speed.to_le_bytes());
    d.extend_from_slice(&addr.to_le_bytes());
    d.extend_from_slice(source.as_bytes());
    d.push(0);
    large(0x8E, &d)
}

fn gpio_int(pin: u16, int_flags: u16, source: &str) -> Vec<u8> {
    let name_offset = 25u16;
    let vendor_offset = name_offset + source.len() as u16 + 1;
    let mut d = alloc::vec![0x01, 0x00, 0x01, 0x00];
    d.extend_from_slice(&int_flags.to_le_bytes());
    d.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]);
    d.extend_from_slice(&23u16.to_le_bytes());
    d.push(0);
    d.extend_from_slice(&name_offset.to_le_bytes());
    d.extend_from_slice(&vendor_offset.to_le_bytes());
    d.extend_from_slice(&0u16.to_le_bytes());
    d.extend_from_slice(&pin.to_le_bytes());
    d.extend_from_slice(source.as_bytes());
    d.push(0);
    large(0x8C, &d)
}

fn ext_irq(gsi: u32, flags: u8) -> Vec<u8> {
    let mut d = alloc::vec![flags, 0x01];
    d.extend_from_slice(&gsi.to_le_bytes());
    large(0x89, &d)
}

fn template(descs: &[Vec<u8>]) -> Vec<u8> {
    let mut b = descs.concat();
    b.extend_from_slice(&[0x79, 0x00]);
    buffer(&b)
}

/// `_DSM` with the HID-over-I2C GUID returning `function_one` for Arg2 == 1.
fn dsm(function_one: &[u8]) -> Vec<u8> {
    let guid = buffer(&GUID);
    let zero = if_(&[0x93, 0x6A, 0x00], &[0xA4, 0x11, 0x03, 0x01, 0x03]);
    let one = if_(&[0x93, 0x6A, 0x01], &cat(&[&[0xA4], function_one]));
    let branch = if_(&cat(&[&[0x93, 0x68], &guid]), &cat(&[&zero, &one]));
    method(b"_DSM", 0x0C, &branch)
}

fn hp_style_touchpad() -> Vec<u8> {
    let crs = template(&[
        i2c_bus(0x2C, 400_000, "\\_SB.PCI0.I2C4"),
        gpio_int(18, 0b010, "\\_SB.GPO1"),
    ]);
    let body = cat(&[
        &name(b"_HID", &string("SYNA3602")),
        &name(b"_CID", &eisa(PNP0C50)),
        &dsm(&[0x0A, 0x20]),
        &name(b"_CRS", &crs),
    ]);
    scope(b"\\/\x03_SB_PCI0I2C4", &device(b"TPD0", &body))
}

#[test]
fn a_vendor_hid_with_the_pnp0c50_cid_is_found_with_every_static_field() {
    let found = parse_hid_devices(&hp_style_touchpad());
    assert_eq!(found.len(), 1);
    let d = &found[0];
    assert_eq!(&d.hid, b"SYNA3602");
    assert_eq!(&d.cid[..7], b"PNP0C50");
    assert_eq!(d.slave_addr, 0x2C);
    assert_eq!(d.speed_hz, 400_000);
    assert_eq!((d.hid_desc_reg, d.desc_reg_from_dsm), (0x20, true));
    assert_eq!(&d.controller, b"I2C4");
    assert_eq!(&d.gpio_controller, b"GPO1");
    assert_eq!(d.interrupt, HidInterrupt::Gpio { pin: 18, level: true, active_high: false });
    assert_eq!(d.device_type, I2cHidDeviceType::Touchpad);
}

#[test]
fn an_intel_reference_placeholder_hid_is_found_by_its_cid_and_its_scope_names_the_bus() {
    // _HID "XXXX0000" is patched at run time; the template's address is zero
    // and the _DSM returns a name whose static value is zero.
    let crs = template(&[i2c_bus(0x00, 400_000, ""), gpio_int(5, 0b011, "\\_SB.GPO0")]);
    let body = cat(&[
        &name(b"_HID", &string("XXXX0000")),
        &name(b"_CID", &string("PNP0C50")),
        &name(b"HID2", &[0x00]),
        &dsm(b"HID2"),
        &name(b"SBFB", &crs),
    ]);
    let aml = scope(b"\\/\x03_SB_PCI0I2C1", &device(b"TPD0", &body));
    let found = parse_hid_devices(&aml);
    assert_eq!(found.len(), 1);
    let d = &found[0];
    assert_eq!(&d.controller, b"I2C1", "the enclosing scope names the controller");
    assert_eq!((d.hid_desc_reg, d.desc_reg_from_dsm), (0x0001, false));
    assert_eq!(d.slave_addr, 0);
    assert_eq!(d.interrupt, HidInterrupt::Gpio { pin: 5, level: false, active_high: false });
}

#[test]
fn a_dsm_returning_a_named_constant_is_resolved() {
    let body = cat(&[
        &name(b"_HID", &string("ELAN0718")),
        &name(b"HIDA", &[0x0A, 0x01]),
        &dsm(b"HIDA"),
        &name(b"_CRS", &template(&[i2c_bus(0x15, 400_000, "\\_SB.PCI0.I2C0")])),
    ]);
    let found = parse_hid_devices(&device(b"TPD1", &body));
    assert_eq!((found[0].hid_desc_reg, found[0].desc_reg_from_dsm), (0x0001, true));
    assert_eq!(found[0].interrupt, HidInterrupt::None);
}

#[test]
fn a_cid_package_is_searched_and_a_static_sta_of_zero_hides_the_device() {
    let cids = cat(&[&[0x12], &pkg(&cat(&[&[0x02], &string("ALPS0001"), &eisa(PNP0C50)]))]);
    let present = cat(&[
        &name(b"_HID", &string("HPQ8001")),
        &name(b"_CID", &cids),
        &name(b"_CRS", &template(&[i2c_bus(0x2C, 100_000, "\\_SB.PCI0.I2C2")])),
    ]);
    let absent = cat(&[&present, &name(b"_STA", &[0x00])]);
    let aml = cat(&[&device(b"TPDA", &present), &device(b"TPDB", &absent)]);
    let found = parse_hid_devices(&aml);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].speed_hz, 100_000);
}

#[test]
fn a_device_on_an_apic_line_reports_its_gsi_trigger_and_polarity() {
    let body = cat(&[
        &name(b"_HID", &string("MSFT0001")),
        &name(b"_CID", &eisa(PNP0C50)),
        // Level, active low: mode bit 1 clear, polarity bit 2 set.
        &name(b"_CRS", &template(&[i2c_bus(0x10, 1_000_000, "\\_SB.I2CA"), ext_irq(0x33, 0x05)])),
    ]);
    let found = parse_hid_devices(&device(b"TPNL", &body));
    assert_eq!(found[0].interrupt, HidInterrupt::Apic { gsi: 0x33, level: true, active_high: false });
    assert_eq!(&found[0].controller, b"I2CA");
}

#[test]
fn devices_that_are_not_hid_over_i2c_are_ignored() {
    let body = cat(&[
        &name(b"_HID", &string("INT33D5")),
        &name(b"_CRS", &template(&[i2c_bus(0x2C, 400_000, "\\_SB.PCI0.I2C1")])),
    ]);
    assert!(parse_hid_devices(&device(b"HIDD", &body)).is_empty());
}

#[test]
fn an_amd_platform_controller_is_named_with_its_window() {
    let mem = {
        let mut d = alloc::vec![0x01];
        d.extend_from_slice(&0xFEDC_3000u32.to_le_bytes());
        d.extend_from_slice(&0x1000u32.to_le_bytes());
        large(0x86, &d)
    };
    let body = cat(&[&name(b"_HID", &string("AMDI0010")), &name(b"_CRS", &template(&[mem]))]);
    let hosts = parse_platform_controllers(&scope(b"_SB_", &device(b"I2CA", &body)));
    assert_eq!(hosts.len(), 1);
    assert_eq!((&hosts[0].name, hosts[0].mmio_base), (b"I2CA", 0xFEDC_3000));
}

#[test]
fn the_enumeration_never_panics_on_arbitrary_bytes() {
    let mut x: u32 = 0x1234_5678;
    for len in 0..512usize {
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            v.push(x as u8);
        }
        let _ = parse_hid_devices(&v);
        let _ = parse_platform_controllers(&v);
        let mut framed = hp_style_touchpad();
        framed.extend_from_slice(&v);
        assert!(!parse_hid_devices(&framed).is_empty());
    }
}
