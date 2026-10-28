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

//! What each network choice gives a guest's stream.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use super::table::{every_default, ANON, SOCKS, UP_DOWN};
use crate::linux::net::guest_route::{path, Net, Path, SOCKS_DOWN};
use crate::pick::{pick, Route};

#[test]
fn anyone_with_net_anon_up_goes_to_net_anon_and_nowhere_else() {
    for nym in [0, SOCKS] {
        assert_eq!(path(Some(ANYONE), nym, ANON), Path::Anyone(ANON));
    }
}

#[test]
fn anyone_with_net_anon_down_waits_for_it_and_never_takes_nym() {
    for nym in [0, SOCKS] {
        let got = path(Some(ANYONE), nym, 0);
        assert!(matches!(got, Path::Wait(Net::Anyone, _)), "nym={nym}: {got:?}");
    }
}

#[test]
fn nym_goes_to_net_socks5_at_its_own_port() {
    for anon in [0, ANON] {
        assert_eq!(path(Some(NYM), SOCKS, anon), Path::Nym(SOCKS));
        assert_eq!(pick(Some(NYM), SOCKS, anon), Route::Nym(SOCKS));
    }
}

#[test]
fn direct_keeps_a_guest_on_nym_and_waits_when_net_socks5_is_not_there() {
    for (nym, anon) in UP_DOWN {
        assert_eq!(pick(Some(DIRECT), nym, anon), Route::Direct);
        let want = if nym != 0 { Path::Nym(nym) } else { Path::Wait(Net::Nym, SOCKS_DOWN) };
        assert_eq!(path(Some(DIRECT), nym, anon), want, "anon={anon}");
    }
}

#[test]
fn an_unreadable_default_is_waited_for_and_never_becomes_nym() {
    for (nym, anon) in UP_DOWN {
        /* What route link's own pick does with it: Nym whenever net.socks5 runs. */
        assert_eq!(matches!(pick(None, nym, anon), Route::Nym(_)), nym != 0);
        assert!(matches!(path(None, nym, anon), Path::Wait(Net::Unread, _)), "{nym} {anon}");
    }
}

#[test]
fn only_the_anyone_default_ever_reaches_net_anon() {
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            if matches!(path(default, nym, anon), Path::Anyone(_)) {
                assert_eq!(default, Some(ANYONE), "nym={nym} anon={anon}");
            }
        }
    }
}
