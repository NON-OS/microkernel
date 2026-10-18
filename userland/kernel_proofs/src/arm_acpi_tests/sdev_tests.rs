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

//! SDEV (offsets recalled from ACPI 6.5 5.2.27, unverified): a namespace
//! device and a PCIe endpoint behind a bridge, each with its handoff flag;
//! the path resolved through the bridge; unknown types and entries whose
//! offsets leave them passed over; an entry past the table refused.

use super::build::*;
use crate::arm_acpi::sdev::{resolve, sdev, SecureDevice};
use crate::arm_acpi::sdt::TableError;

fn acpi(flags: u8, path: &[u8]) -> Vec<u8> {
    let len = 16 + path.len();
    let head = [vec![0, flags], u16le(len as u16), u16le(16), u16le(path.len() as u16)].concat();
    [head, vec![0; 8], path.to_vec()].concat()
}

fn pci(flags: u8, bus: u16, hops: &[(u8, u8)]) -> Vec<u8> {
    let len = 16 + 2 * hops.len();
    let head = [vec![1, flags], u16le(len as u16), u16le(0), u16le(bus), u16le(16)].concat();
    let path: Vec<u8> = hops.iter().flat_map(|(d, f)| [*d, *f]).collect();
    [head, u16le(path.len() as u16), vec![0; 4], path].concat()
}

#[test]
fn a_namespace_device_and_a_pcie_endpoint() {
    let t = sdev(&sdt(
        b"SDEV",
        1,
        &[acpi(1, b"\\_SB.SPI1\0"), pci(0, 0, &[(0x1C, 0), (0, 0)])].concat(),
    ));
    let e = t.unwrap();
    assert_eq!(e.len(), 2);
    assert_eq!((&e[0].device, e[0].handoff), (&SecureDevice::Acpi(b"\\_SB.SPI1".to_vec()), true));
    let SecureDevice::Pci { segment, start_bus, path } = &e[1].device else { panic!("pci") };
    assert_eq!((*segment, *start_bus, e[1].handoff), (0, 0, false));
    let bridge = |bus, dev, func| ((bus, dev, func) == (0, 0x1C, 0)).then_some(3);
    assert_eq!(resolve(*start_bus, path, bridge), Some((3, 0, 0)), "behind the root port");
    assert_eq!(resolve(*start_bus, path, |_, _, _| None), None, "a bridge that does not answer");
}

#[test]
fn passed_over_and_refused() {
    let unknown = [vec![7, 0], u16le(8), vec![0; 4]].concat();
    let mut inside = acpi(0, b"\\X\0");
    inside[4..6].copy_from_slice(&u16le(2));
    let no_path = pci(0, 0, &[]);
    let t =
        sdev(&sdt(b"SDEV", 1, &[unknown, inside, no_path, acpi(0, b"\\Y\0")].concat())).unwrap();
    assert_eq!(t.len(), 1, "only the sound entry");
    let mut past = sdt(b"SDEV", 1, &acpi(0, b"\\Z\0"));
    past[38..40].copy_from_slice(&u16le(200));
    seal(&mut past);
    assert_eq!(sdev(&past), Err(TableError::Structure(36)));
}
