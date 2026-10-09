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
//! it: the verdict, then one byte per gate.

/// The six gates, in the order the capsule writes them after the verdict.
pub const GATES: [&[u8]; 6] = [
    b"index signature",
    b"package present",
    b"publisher signature",
    b"operator validation",
    b"architecture",
    b"attestation",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Readiness {
    pub install_ready: bool,
    pub gates: [bool; 6],
}

pub fn parse_readiness(body: &[u8]) -> Option<Readiness> {
    let bytes = body.get(..1 + GATES.len())?;
    let mut gates = [false; 6];
    for (g, b) in gates.iter_mut().zip(&bytes[1..]) {
        *g = *b != 0;
    }
    Some(Readiness { install_ready: bytes[0] != 0, gates })
}
