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

//! The network a guest's stream leaves through follows the person's choice.
//!
//! It always went to the mixnet, so under the Anyone network every guest
//! connection failed and a guest's traffic left on a network nobody chose.
//! Held here over every default the policy store could hold, or none, with
//! each network up or down, through nonos_route_link's own pick: Anyone goes
//! to net.anon or nowhere, Nym and an unreadable default go to the mixnet,
//! and Direct goes to the mixnet too, since a guest never reaches the
//! network directly.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::linux::net::guest_route::{path, Path};
use crate::pick::{pick, Route, ANYONE_DOWN};

/// Ports of net.socks5 and net.anon as service lookup would give them.
const SOCKS: u32 = 4908;
const ANON: u32 = 4484;

/// Not asked (None), then every byte the store could answer.
fn every_default() -> impl Iterator<Item = Option<u8>> {
    core::iter::once(None).chain((0..=u8::MAX).map(Some))
}

/// (Nym up, Anyone up), each 0 when that network does not run.
const UP_DOWN: [(u32, u32); 4] = [(0, 0), (SOCKS, 0), (0, ANON), (SOCKS, ANON)];

fn guest_path(default: Option<u8>, nym: u32, anon: u32) -> Path {
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
fn anyone_with_net_anon_up_goes_to_net_anon_and_nowhere_else() {
    for nym in [0, SOCKS] {
        assert_eq!(guest_path(Some(ANYONE), nym, ANON), Path::Anyone(ANON));
    }
}

#[test]
fn anyone_with_net_anon_down_is_unreachable_and_says_which_network() {
    for nym in [0, SOCKS] {
        let got = guest_path(Some(ANYONE), nym, 0);
        assert_eq!(got, Path::Unreachable(ANYONE_DOWN), "nym={nym}");
        /* Not the mixnet, though it runs: nothing else is tried. */
        assert_ne!(got, Path::Mixnet);
        let Path::Unreachable(why) = got else { unreachable!() };
        assert!(why.contains("Anyone"), "the [LINUX] line names the network: {why}");
    }
}

#[test]
fn nym_and_an_unreadable_default_go_to_the_mixnet_up_or_down() {
    for (nym, anon) in UP_DOWN {
        assert_eq!(guest_path(Some(NYM), nym, anon), Path::Mixnet, "nym={nym} anon={anon}");
        assert_eq!(guest_path(None, nym, anon), Path::Mixnet, "nym={nym} anon={anon}");
    }
}

#[test]
fn direct_keeps_a_guest_on_the_mixnet() {
    for (nym, anon) in UP_DOWN {
        assert_eq!(pick(Some(DIRECT), nym, anon), Route::Direct);
        assert_eq!(guest_path(Some(DIRECT), nym, anon), Path::Mixnet, "nym={nym} anon={anon}");
    }
}

#[test]
fn only_the_anyone_default_ever_leaves_the_mixnet() {
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            if guest_path(default, nym, anon) != Path::Mixnet {
                assert_eq!(default, Some(ANYONE), "nym={nym} anon={anon}");
            }
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
