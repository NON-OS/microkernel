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
//! Each id's MAC, and what differs by MAC: the receive-address count, Go
//! Link Disconnect and the paged PHY.

use crate::facts::ids::{part_of, IGB_PARTS};
use crate::facts::parts::Part;

fn ids(of: Part) -> Vec<u16> {
    IGB_PARTS.iter().filter(|(_, p, _)| *p == of).map(|(id, ..)| *id).collect()
}

#[test]
fn the_macs_are_linux_s() {
    assert_eq!(ids(Part::I82575), [0x10A7, 0x10A9, 0x10D6]);
    let i82576 = [0x10C9, 0x150A, 0x1518, 0x10E6, 0x10E7, 0x150D, 0x1526, 0x10E8];
    assert_eq!(ids(Part::I82576), i82576);
    let i82580 = [0x150E, 0x150F, 0x1527, 0x1510, 0x1511, 0x1516, 0x0438, 0x043A, 0x043C, 0x0440];
    assert_eq!(ids(Part::I82580), i82580, "the DH89xxCC runs as an 82580");
    assert_eq!(ids(Part::I350), [0x1521, 0x1522, 0x1523, 0x1524]);
    assert_eq!(ids(Part::I354), [0x1F40, 0x1F41, 0x1F45]);
    assert_eq!(ids(Part::I210), [0x1533, 0x1536, 0x1537, 0x1538, 0x157B, 0x157C]);
    assert_eq!(ids(Part::I211), [0x1539]);
    assert_eq!(part_of(0x10C9), Some(Part::I82576), "QEMU's igb");
    assert_eq!(part_of(0x15F2), None);
}

#[test]
fn what_differs_by_mac() {
    let rar = |p: Part| p.rar_entries();
    assert_eq!([rar(Part::I82575), rar(Part::I210), rar(Part::I211)], [16, 16, 16]);
    assert_eq!([rar(Part::I82576), rar(Part::I82580)], [24, 24]);
    assert_eq!([rar(Part::I350), rar(Part::I354)], [32, 32]);
    let go: Vec<Part> = all().into_iter().filter(|p| p.has_go_link_disconnect()).collect();
    assert_eq!(go, [Part::I82580, Part::I350, Part::I210, Part::I211]);
    let paged: Vec<Part> = all().into_iter().filter(|p| p.paged_phy()).collect();
    assert_eq!(paged, [Part::I210, Part::I211]);
}

fn all() -> [Part; 7] {
    use Part::*;
    [I82575, I82576, I82580, I350, I354, I210, I211]
}

/// The I225/I226 is igc's and the 8254x/8257x parts e1000's and e1000e's;
/// binding any of them would run this sequence on the wrong silicon.
#[test]
fn igc_and_e1000_parts_are_not_taken() {
    for id in [0x15F2u16, 0x15F3, 0x125B, 0x125C, 0x100E, 0x100F, 0x10D3, 0x153A, 0x15B8] {
        assert!(IGB_PARTS.iter().all(|(i, ..)| *i != id), "{id:#06x} is not an igb part");
    }
}
