// NONOS Operating System (AGPL-3.0-or-later)
//! The boot census's thermal zone count: each ThermalZone whose length and
//! name hold together inside its block, over every block; never a pattern
//! that runs past its table.

use crate::arch::x86_64::acpi::aml::thermal_zones::{count, count_blocks};

/// ThermalZone (NAME) { body }: 5B 85, a one-byte PkgLength, the name.
fn zone(name: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let len = 1 + 4 + body.len();
    assert!(len < 0x40, "one-byte PkgLength");
    [&[0x5B, 0x85, len as u8][..], name, body].concat()
}

/// Scope (\_TZ) { zones }.
fn scope(body: &[u8]) -> Vec<u8> {
    let len = 1 + 4 + body.len();
    [&[0x10, len as u8][..], b"_TZ_", body].concat()
}

#[test]
fn each_zone_in_the_scope_counts() {
    let two = scope(&[zone(b"TZ00", &[0xA4, 0x00]), zone(b"TZ01", &[])].concat());
    assert_eq!(count(&two), 2);
    assert_eq!(count(&[]), 0);
}

#[test]
fn a_pattern_that_does_not_hold_together_is_not_counted() {
    // A length past the block's end.
    assert_eq!(count(&[0x5B, 0x85, 0x30, b'T', b'Z', b'0', b'0']), 0);
    // The bytes inside a buffer that happen to read 5B 85 with no name.
    assert_eq!(count(&[0x11, 0x05, 0x0A, 0x02, 0x5B, 0x85, 0x01]), 0);
}

#[test]
fn the_blocks_add_up_and_the_count_is_bounded() {
    let dsdt = scope(&zone(b"TZ00", &[]));
    let ssdt = zone(b"CPUZ", &[]);
    assert_eq!(count_blocks([dsdt.as_slice(), ssdt.as_slice()]), 2);
    let many: Vec<u8> = (0..100).flat_map(|_| zone(b"TZZZ", &[])).collect();
    assert_eq!(count(&many), 64, "stops at its bound");
}
