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

//! The ceilings to hand the kernel: each processor's lowest limit among the
//! zones whose _PSL names it, and 100 for one that no zone limits now but
//! the kernel last heard a lower limit for. Only changes are sent. Pure.

use alloc::vec::Vec;

pub const FULL: u8 = 100;

/// `zones`: each zone's processors and its limit; `sent`: the limits the
/// kernel holds. The changes, by processor UID.
pub fn changes(zones: &[(&[u64], u8)], sent: &[(u64, u8)]) -> Vec<(u64, u8)> {
    let mut want: Vec<(u64, u8)> = Vec::new();
    for (uids, limit) in zones {
        for uid in uids.iter() {
            match want.iter_mut().find(|w| w.0 == *uid) {
                Some(w) => w.1 = w.1.min(*limit),
                None => want.push((*uid, (*limit).min(FULL))),
            }
        }
    }
    for (uid, _) in sent {
        if !want.iter().any(|w| w.0 == *uid) {
            want.push((*uid, FULL));
        }
    }
    let held = |uid: u64| sent.iter().find(|s| s.0 == uid).map_or(FULL, |s| s.1);
    want.retain(|(uid, limit)| *limit != held(*uid));
    want.sort_unstable();
    want
}

/// What the kernel holds once it took `limit` for `uid`: 100 is no entry.
pub fn record(sent: &mut Vec<(u64, u8)>, uid: u64, limit: u8) {
    sent.retain(|s| s.0 != uid);
    if limit < FULL {
        sent.push((uid, limit));
    }
}
