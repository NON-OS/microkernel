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

//! Lists and guests held to them, for the reach tests.

use crate::app::identity::AppIdentity;
use nonos_linux_policy::dests::dests_hash;
use nonos_linux_policy::mask::{NET_ANON, TTY};
use nonos_linux_policy::{unlisted, LinuxAppPolicy};

/// A list of names, each on port 443, as a canonical list writes them.
pub fn names(names: &[&str]) -> &'static [u8] {
    let mut entries: Vec<Vec<u8>> = names
        .iter()
        .map(|n| [&[b'N', 0, 1, 187, 1, 187, n.len() as u8][..], n.as_bytes()].concat())
        .collect();
    entries.sort();
    let mut list = (entries.len() as u16).to_be_bytes().to_vec();
    entries.iter().for_each(|e| list.extend_from_slice(e));
    Box::leak(list.into_boxed_slice())
}

pub fn held(program: u8, list: &'static [u8]) -> AppIdentity {
    let p = LinuxAppPolicy {
        mask: NET_ANON | TTY,
        net_dests_hash: dests_hash(list),
        ..unlisted([program; 32])
    };
    AppIdentity::bound(p, Some(list))
}
