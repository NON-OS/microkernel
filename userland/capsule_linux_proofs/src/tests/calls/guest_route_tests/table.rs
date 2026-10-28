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

//! Every default and network state, and the one path each gives.

use nonos_policy_proto::route::{ANYONE, DIRECT};

use crate::linux::net::guest_route::{path, Net, Path, SOCKS_DOWN, UNREAD};
use crate::pick::{ANYONE_DOWN, NYM_DOWN};

/// Ports of net.socks5 and net.anon as service lookup would give them.
pub(super) const SOCKS: u32 = 4908;
pub(super) const ANON: u32 = 4484;

/// Not asked (None), then every byte the store could answer.
pub(super) fn every_default() -> impl Iterator<Item = Option<u8>> {
    core::iter::once(None).chain((0..=u8::MAX).map(Some))
}

/// (Nym up, Anyone up), each 0 when that network does not run.
pub(super) const UP_DOWN: [(u32, u32); 4] = [(0, 0), (SOCKS, 0), (0, ANON), (SOCKS, ANON)];

#[test]
fn the_whole_table() {
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            let want = match default {
                None => Path::Wait(Net::Unread, UNREAD),
                Some(ANYONE) if anon != 0 => Path::Anyone(anon),
                Some(ANYONE) => Path::Wait(Net::Anyone, ANYONE_DOWN),
                _ if nym != 0 => Path::Nym(nym),
                Some(DIRECT) => Path::Wait(Net::Nym, SOCKS_DOWN),
                Some(_) => Path::Wait(Net::Nym, NYM_DOWN),
            };
            assert_eq!(path(default, nym, anon), want, "{default:?} nym={nym} anon={anon}");
        }
    }
}

#[test]
fn every_wait_names_its_network_and_a_reason() {
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            if let Path::Wait(net, why) = path(default, nym, anon) {
                assert!(!why.is_empty());
                let named = match net {
                    Net::Anyone => why.contains("Anyone"),
                    Net::Nym => why.contains("Nym") || why.contains("net.socks5"),
                    Net::Unread => why.contains("default network"),
                };
                assert!(named, "{net:?}: {why}");
            }
        }
    }
}
