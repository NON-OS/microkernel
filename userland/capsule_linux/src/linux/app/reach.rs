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

//! Where a guest may reach out to, decided from its policies alone, so the
//! host proofs hold each rule as it ships. A box with NET_ANON and a zero
//! net_dests_hash reaches anything over the anonymity network; one naming a
//! list reaches what the list allows (nonos_linux_policy::dests), and after
//! an exec, only what every list on the way allows.

use nonos_linux_policy::dests::{allows, check, dests_hash, Target};
use nonos_linux_policy::mask::{grants, NET_ANON};

/// The lists a guest is held to; an empty one holds it to nothing.
pub type Lists = [&'static [u8]; 2];

pub const ANYWHERE: Lists = [&[], &[]];

/// The mask and lists a freshly bound box is served. A policy naming a list
/// this capsule does not hold, or one that is not canonical or does not hash
/// to it, is served without NET_ANON.
pub fn reach_of(mask: u64, net_dests_hash: &[u8; 32], list: Option<&'static [u8]>) -> (u64, Lists) {
    if *net_dests_hash == [0; 32] {
        return (mask, ANYWHERE);
    }
    match list.filter(|l| check(l).is_ok() && dests_hash(l) == *net_dests_hash) {
        Some(l) => (mask, [l, &[]]),
        None => (mask & !NET_ANON, ANYWHERE),
    }
}

/// The lists after an exec: those before it and the new program's, each
/// once. Two are kept; when a third would be, None, and the guest is then
/// served without NET_ANON.
pub fn chain(before: &Lists, own: &Lists) -> Option<Lists> {
    let mut out = ANYWHERE;
    let mut held = 0;
    for list in before.iter().chain(own).filter(|l| !l.is_empty()) {
        if out[..held].contains(list) {
            continue;
        }
        *out.get_mut(held)? = list;
        held += 1;
    }
    Some(out)
}

/// A stream out of the family to `to` on `port`.
pub fn may_dial(mask: u64, lists: &Lists, to: Target<'_>, port: u16) -> bool {
    grants(mask, NET_ANON) && lists.iter().all(|l| l.is_empty() || allows(l, to, port))
}
