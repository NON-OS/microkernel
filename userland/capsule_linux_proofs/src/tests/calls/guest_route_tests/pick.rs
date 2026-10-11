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

//! What each network choice picks for a guest's stream.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use super::table::{every_default, guest_path, ANON, SOCKS, UP_DOWN};
use crate::linux::net::guest_route::Path;
use crate::pick::{pick, Route, ANYONE_DOWN};

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
