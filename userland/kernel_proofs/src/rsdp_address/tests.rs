// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::modules_acpi::AcpiRsdp;

fn rsdp(revision: u8, xsdt: Option<u64>) -> AcpiRsdp {
    AcpiRsdp {
        signature: *b"RSD PTR ",
        checksum: 0,
        oem_id: *b"NONOS ",
        revision,
        rsdt_address: 0x000E_0000,
        length: None,
        xsdt_address: xsdt,
        extended_checksum: None,
        reserved: None,
    }
}

#[test]
fn xsdt_is_used_only_from_revision_two() {
    assert_eq!(rsdp(0, Some(0xDEAD_0000)).table_address(), 0x000E_0000);
    assert_eq!(rsdp(1, Some(0xDEAD_0000)).table_address(), 0x000E_0000);
    assert_eq!(rsdp(2, Some(0xDEAD_0000)).table_address(), 0xDEAD_0000);
    assert_eq!(rsdp(2, Some(0)).table_address(), 0x000E_0000);
    assert_eq!(rsdp(2, None).table_address(), 0x000E_0000);
}

fn byte_sum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |a, &b| a.wrapping_add(b))
}

fn with_extension(mut r: AcpiRsdp, length: u32, reserved: [u8; 3]) -> AcpiRsdp {
    let xsdt = r.xsdt_address.unwrap_or(0);
    let sum = byte_sum(&r.signature)
        .wrapping_add(r.checksum)
        .wrapping_add(byte_sum(&r.oem_id))
        .wrapping_add(r.revision)
        .wrapping_add(byte_sum(&r.rsdt_address.to_le_bytes()))
        .wrapping_add(byte_sum(&length.to_le_bytes()))
        .wrapping_add(byte_sum(&xsdt.to_le_bytes()))
        .wrapping_add(byte_sum(&reserved));
    r.length = Some(length);
    r.reserved = Some(reserved);
    r.extended_checksum = Some(0u8.wrapping_sub(sum));
    r
}

#[test]
fn extended_checksum_needs_every_acpi2_field_and_the_reserved_bytes() {
    assert!(!rsdp(2, None).verify_extended_checksum());
    let good = with_extension(rsdp(2, Some(0xDEAD_0000)), 36, [1, 2, 3]);
    assert!(good.verify_extended_checksum());
    let mut tampered = good.clone();
    tampered.reserved = Some([1, 2, 4]);
    assert!(!tampered.verify_extended_checksum());
    assert!(!with_extension(rsdp(2, Some(0xDEAD_0000)), 20, [0; 3]).verify_extended_checksum());
    assert!(rsdp(1, None).verify_extended_checksum());
}

#[test]
fn a_declared_length_other_than_36_is_refused() {
    for length in [37u32, 40, 4096, u32::MAX] {
        let longer = with_extension(rsdp(2, Some(0xDEAD_0000)), length, [0; 3]);
        assert!(!longer.verify_extended_checksum(), "length {length}");
    }
}
