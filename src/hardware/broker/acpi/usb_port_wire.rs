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

//! A USB host's port facts as the ACPI capsule hands them over and drivers
//! read them back: eight bytes a port, at most `MAX_PORTS`. Pure: the
//! capsule and `kernel_proofs` mount this file as it is.
//!
//! Each port: the port of the hub above it (0 for the root hub), its port
//! number, flags (`UPC`, `CONNECTABLE`, `PLD`, `VISIBLE`, `LID`, `DOCK`,
//! then the reach in bits 7:6: 0 unknown, 1 internal, 2 external), the
//! _UPC connector type, the _PLD panel (0 top to 6 unknown), three zeros.

use alloc::vec::Vec;

pub const PORT_LEN: usize = 8;
pub const MAX_PORTS: usize = 64;
pub const UPC: u8 = 1;
pub const CONNECTABLE: u8 = 2;
pub const PLD: u8 = 4;
pub const VISIBLE: u8 = 8;
pub const LID: u8 = 16;
pub const DOCK: u8 = 32;
pub const REACH_SHIFT: u8 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PortFact {
    pub parent: u8,
    pub port: u8,
    pub flags: u8,
    pub connector: u8,
    pub panel: u8,
}

impl PortFact {
    /// 0 unknown, 1 internal, 2 external.
    pub fn reach(&self) -> u8 {
        self.flags >> REACH_SHIFT
    }
}

pub fn encode(ports: &[PortFact]) -> Option<Vec<u8>> {
    let b: Vec<u8> = ports
        .iter()
        .flat_map(|p| [p.parent, p.port, p.flags, p.connector, p.panel, 0, 0, 0])
        .collect();
    decode(&b).filter(|d| d == ports).map(|_| b)
}

/// None unless every port holds: whole records, no more than `MAX_PORTS`,
/// reserved bytes zero, a reach of 0 to 2, a panel of 0 to 6, and no
/// facts claimed from a _UPC or _PLD the port does not have.
pub fn decode(b: &[u8]) -> Option<Vec<PortFact>> {
    if !b.len().is_multiple_of(PORT_LEN) || b.len() / PORT_LEN > MAX_PORTS {
        return None;
    }
    let one = |r: &[u8]| {
        let p = PortFact { parent: r[0], port: r[1], flags: r[2], connector: r[3], panel: r[4] };
        let upc_facts = p.flags & CONNECTABLE == 0 || p.flags & UPC != 0;
        let pld_facts = p.flags & (VISIBLE | LID | DOCK) == 0 || p.flags & PLD != 0;
        let ok = r[5..] == [0, 0, 0] && p.reach() <= 2 && p.panel <= 6 && upc_facts && pld_facts;
        ok.then_some(p)
    };
    b.chunks_exact(PORT_LEN).map(one).collect()
}
