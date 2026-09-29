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

//! Why a listing cannot be installed.

use super::wire::call;

const OP_INSTALL_READY: u16 = 5;

/// The six gates, in the order the capsule writes them after the verdict.
pub const GATES: [&[u8]; 6] = [
    b"index signature",
    b"package present",
    b"publisher signature",
    b"operator validation",
    b"architecture",
    b"attestation",
];

#[derive(Clone, Copy)]
pub struct Readiness {
    pub install_ready: bool,
    pub gates: [bool; 6],
}

pub fn fetch(port: u32, request_id: u32, listing: &[u8], release: &[u8]) -> Option<Readiness> {
    let mut body = alloc::vec::Vec::with_capacity(8 + listing.len() + release.len());
    body.extend_from_slice(&(listing.len() as u32).to_le_bytes());
    body.extend_from_slice(listing);
    body.extend_from_slice(&(release.len() as u32).to_le_bytes());
    body.extend_from_slice(release);
    let out = call(port, OP_INSTALL_READY, request_id, &body)?;
    // Seven bytes: the verdict then one per gate.
    if out.len() < 1 + GATES.len() {
        return None;
    }
    let mut gates = [false; 6];
    for (i, g) in gates.iter_mut().enumerate() {
        *g = out[1 + i] != 0;
    }
    Some(Readiness { install_ready: out[0] != 0, gates })
}
