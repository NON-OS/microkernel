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

//! A box's policy hash: one for each box, whatever program runs in it. The
//! preset, the services, the folders and the destination list, each in its
//! canonical form, so `allow:/tmp/b,/tmp/a` and `allow:/tmp/a,/tmp/b` hash alike.

use nonos_linux_policy::dests::dests_hash;

use super::preset::{BoxPolicy, Preset};

pub const DOMAIN: &[u8] = b"NONOS.linux.box.v1";

pub fn box_hash(policy: &BoxPolicy) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(DOMAIN);
    h.update(&[match policy.preset {
        Preset::Strict => 1,
        Preset::Offline => 2,
        Preset::Allow => 3,
        Preset::Learn => 4,
    }]);
    h.update(&policy.mask.to_be_bytes());
    h.update(&(policy.folders.len() as u32).to_be_bytes());
    for f in &policy.folders {
        h.update(&(f.len() as u32).to_be_bytes());
        h.update(f);
    }
    let dests = match policy.dests.is_empty() {
        true => [0; 32],
        false => dests_hash(&policy.dests),
    };
    h.update(&dests);
    *h.finalize().as_bytes()
}

pub fn hex(bytes: &[u8]) -> alloc::string::String {
    bytes.iter().map(|b| alloc::format!("{b:02x}")).collect()
}
