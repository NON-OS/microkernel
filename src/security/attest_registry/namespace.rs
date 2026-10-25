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

//! Each running capsule's manifest namespace, as BLAKE3 of the string, kept
//! beside its registry entry and dropped with it. Not folded into the root:
//! the root says what was proved; this only says whose bound machine keys a
//! capsule may derive (`security::tpm::machine_key`, BOUND_PREFIX).

use spin::Mutex;

use super::authority::authority_of;
use super::table::MAX_ATTESTED;
use crate::security::dev_roots::Authority;

struct Namespaces {
    pids: [u32; MAX_ATTESTED],
    hashes: [[u8; 32]; MAX_ATTESTED],
    used: usize,
}

static NAMESPACES: Mutex<Namespaces> =
    Mutex::new(Namespaces { pids: [0; MAX_ATTESTED], hashes: [[0; 32]; MAX_ATTESTED], used: 0 });

/// Record `pid`'s namespace hash at spawn. A full table records nothing,
/// and that capsule's bound labels are refused: never another's key.
pub fn record_namespace(pid: u32, namespace_hash: [u8; 32]) {
    let mut t = NAMESPACES.lock();
    let at = match t.pids[..t.used].iter().position(|&p| p == pid) {
        Some(i) => i,
        None if t.used < MAX_ATTESTED => {
            t.used += 1;
            t.used - 1
        }
        None => return,
    };
    t.pids[at] = pid;
    t.hashes[at] = namespace_hash;
}

pub(super) fn forget_namespace(pid: u32) {
    let mut t = NAMESPACES.lock();
    if let Some(i) = t.pids[..t.used].iter().position(|&p| p == pid) {
        let last = t.used - 1;
        t.pids[i] = t.pids[last];
        t.hashes[i] = t.hashes[last];
        t.pids[last] = 0;
        t.hashes[last] = [0; 32];
        t.used = last;
    }
}

/// `pid`'s namespace hash, only while the vendor root's proof of it stands.
pub fn bound_namespace_of(pid: u32) -> Option<[u8; 32]> {
    if !matches!(authority_of(pid), Some(Authority::Vendor)) {
        return None;
    }
    let t = NAMESPACES.lock();
    t.pids[..t.used].iter().position(|&p| p == pid).map(|i| t.hashes[i])
}
