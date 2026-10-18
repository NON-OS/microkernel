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

use nonos_policy_proto::route::ANYONE;

use crate::linux::net::guest_route::{path, Path};
use crate::pick::{pick, Route, ANYONE_DOWN};

/// Ports of net.socks5 and net.anon as service lookup would give them.
pub(super) const SOCKS: u32 = 4908;
pub(super) const ANON: u32 = 4484;

/// Not asked (None), then every byte the store could answer.
pub(super) fn every_default() -> impl Iterator<Item = Option<u8>> {
    core::iter::once(None).chain((0..=u8::MAX).map(Some))
}

/// (Nym up, Anyone up), each 0 when that network does not run.
pub(super) const UP_DOWN: [(u32, u32); 4] = [(0, 0), (SOCKS, 0), (0, ANON), (SOCKS, ANON)];

pub(super) fn guest_path(default: Option<u8>, nym: u32, anon: u32) -> Path {
    path(pick(default, nym, anon))
}

#[test]
fn the_whole_table() {
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            let want = match default {
                Some(ANYONE) if anon != 0 => Path::Anyone(anon),
                Some(ANYONE) => Path::Unreachable(ANYONE_DOWN),
                _ => Path::Mixnet,
            };
            assert_eq!(guest_path(default, nym, anon), want, "{default:?} nym={nym} anon={anon}");
        }
    }
}

#[test]
fn every_route_pick_can_give_has_one_path() {
    assert_eq!(path(Route::Nym(SOCKS)), Path::Mixnet);
    assert_eq!(path(Route::Direct), Path::Mixnet);
    assert_eq!(path(Route::Anon(ANON)), Path::Anyone(ANON));
    assert_eq!(path(Route::Down(ANYONE_DOWN)), Path::Unreachable(ANYONE_DOWN));
    assert_eq!(path(Route::Down(crate::pick::NYM_DOWN)), Path::Mixnet);
    assert_eq!(path(Route::Down(crate::pick::UNREAD)), Path::Mixnet);
}
