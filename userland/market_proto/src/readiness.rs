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

//! Why a listing can or cannot be installed, as `OP_INSTALL_READY` returns
//! it: the verdict, one byte per gate, then the bond's code (v1.4.35).

use crate::bond_code::BondCode;

/// The gates, in the order the capsule writes them after the verdict. The
/// four after six are schema 3's (plan/25), the last the bond (v1.4.23).
pub const GATES: [&[u8]; 11] = [
    b"index signature",
    b"package present",
    b"publisher signature",
    b"operator validation",
    b"architecture",
    b"attestation",
    b"commerce",
    b"release attest",
    b"guest policy",
    b"price",
    b"bond",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Readiness {
    pub install_ready: bool,
    pub gates: [bool; 11],
    /// Why the bond gate stands as it does.
    pub bond: BondCode,
}

/// The verdict, the gates and the bond's code.
pub const READINESS_LEN: usize = 1 + GATES.len() + 1;

/// None unless the reply is exactly READINESS_LEN bytes naming a known
/// bond code: a byte past the bond's code is no readiness either.
pub fn parse_readiness(body: &[u8]) -> Option<Readiness> {
    let row: &[u8; READINESS_LEN] = body.try_into().ok()?;
    let (verdict, rest) = row.split_first()?;
    let (bond, gate_bytes) = rest.split_last()?;
    let bond = BondCode::from_byte(*bond)?;
    let mut gates = [false; 11];
    for (g, b) in gates.iter_mut().zip(gate_bytes) {
        *g = *b != 0;
    }
    Some(Readiness { install_ready: *verdict != 0, gates, bond })
}
