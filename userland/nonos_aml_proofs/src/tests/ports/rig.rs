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

//! A laptop's USB ports as firmware describes them (ACPI 6.5, 9.14 and
//! 6.1.8): a Type-A socket on the left, a proprietary port wired to a
//! Bluetooth module, a port with no connector, a camera on the lid, a
//! Type-C port behind which firmware describes a hub's port, and ports
//! whose _PLD buffers are too short for their revision.

use crate::enc::*;

/// A _PLD buffer of `len` bytes: revision, then bit 64 user visible, 65
/// dock, 66 lid, 69:67 panel.
pub fn pld_buf(rev: u8, len: usize, visible: bool, lid: bool, panel: u8) -> Vec<u8> {
    let mut b = vec![0u8; len];
    if let Some(first) = b.first_mut() {
        *first = rev | 0x80;
    }
    if len > 8 {
        b[8] = u8::from(visible) | u8::from(lid) << 2 | (panel & 7) << 3;
    }
    b
}

fn pld(b: &[u8]) -> Vec<u8> {
    name_obj("_PLD", &package(1, &[buffer(b.len() as u64, b)]))
}

fn upc(connectable: u64, kind: u64) -> Vec<u8> {
    name_obj("_UPC", &package(4, &[int(connectable), int(kind), int(0), int(0)]))
}

fn port(path: &str, adr: u64, rest: &[Vec<u8>]) -> Vec<u8> {
    device(path, &cat(&[&name_obj("_ADR", &int(adr)), &rest.concat()]))
}

pub fn laptop() -> Vec<u8> {
    let h = "\\_SB.PCI0.XHC.RHUB";
    cat(&[
        &device("\\_SB.PCI0", &[]),
        &port("\\_SB.PCI0.XHC", 0x0014_0000, &[]),
        &port(h, 0, &[]),
        &port(&format!("{h}.HS01"), 1, &[upc(1, 0x03), pld(&pld_buf(2, 20, true, false, 2))]),
        &port(&format!("{h}.HS02"), 2, &[upc(1, 0xFF), pld(&pld_buf(2, 20, false, false, 6))]),
        &port(&format!("{h}.HS03"), 3, &[upc(0, 0x00)]),
        &port(&format!("{h}.HS04"), 4, &[upc(1, 0xFF), pld(&pld_buf(1, 16, true, true, 4))]),
        &port(&format!("{h}.SS05"), 5, &[upc(1, 0x09), pld(&pld_buf(2, 20, true, false, 3))]),
        &port(&format!("{h}.SS05.HB01"), 1, &[upc(1, 0x00)]),
        &port(&format!("{h}.HS06"), 6, &[pld(&pld_buf(1, 8, true, false, 0))]),
        &port(&format!("{h}.HS07"), 7, &[pld(&pld_buf(2, 16, true, false, 0))]),
    ])
}
