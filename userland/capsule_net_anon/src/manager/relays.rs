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

//! Joining consensus entries to the microdescriptors they name.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::consensus::Entry;
use crate::directory::microdesc::Microdesc;
use crate::path::{Ports, Relay};

/// Build the usable relay set from both halves, and the policy summaries of
/// the exits among them.
pub fn join(entries: &[Entry], found: &[([u8; 32], Microdesc)]) -> (Vec<Relay>, Ports) {
    let mut out = Vec::with_capacity(entries.len());
    let mut ports = Vec::new();
    for entry in entries {
        let Ok(at) = found.binary_search_by(|(d, _)| d.cmp(&entry.microdesc_digest)) else {
            continue;
        };
        let (_, micro) = &found[at];
        if entry.flags.exit {
            ports.push((entry.rsa_identity, micro.ports.clone()));
        }
        out.push(Relay {
            address: entry.address,
            or_port: entry.or_port,
            rsa_identity: entry.rsa_identity,
            ed25519_identity: micro.ed25519_identity,
            ntor_onion_key: micro.ntor_onion_key,
            flags: entry.flags,
            weight: entry.weight,
            exits_web: micro.exits_web,
        });
    }
    (out, Ports::new(ports))
}
