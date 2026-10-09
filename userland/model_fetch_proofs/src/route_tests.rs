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

/*
 * The network a model download leaves through: the one the person chose,
 * or none. A direct connection names this machine to the mirror, so it is
 * taken only when it is the choice, never because another network is not
 * up or the policy store could not be asked.
 */

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::net::pick::{pick, Route, ANYONE_DOWN, NYM_DOWN, UNREAD};

const SOCKS: u32 = 7;
const ANON: u32 = 9;

#[test]
fn the_chosen_network_is_the_one_taken() {
    assert_eq!(pick(Some(NYM), SOCKS, ANON), Route::Nym(SOCKS));
    assert_eq!(pick(Some(ANYONE), SOCKS, ANON), Route::Anon(ANON));
    assert_eq!(pick(Some(DIRECT), SOCKS, ANON), Route::Direct);
    assert_eq!(pick(Some(DIRECT), 0, 0), Route::Direct);
    /* No answer from the store, or one this build does not know: the default, the mixnet. */
    assert_eq!(pick(None, SOCKS, ANON), Route::Nym(SOCKS));
    assert_eq!(pick(Some(200), SOCKS, ANON), Route::Nym(SOCKS));
}

#[test]
fn a_network_that_is_not_running_is_no_route_and_nothing_falls_back() {
    assert_eq!(pick(Some(NYM), 0, ANON), Route::Down(NYM_DOWN));
    assert_eq!(pick(Some(NYM), 0, 0), Route::Down(NYM_DOWN));
    assert_eq!(pick(Some(ANYONE), SOCKS, 0), Route::Down(ANYONE_DOWN));
    /*
     * The store could not be asked and the mixnet is down: before, this went
     * to the Anyone network, or with neither up, straight to the mirror.
     */
    assert_eq!(pick(None, 0, ANON), Route::Down(UNREAD));
    assert_eq!(pick(None, 0, 0), Route::Down(UNREAD));
}

/* Every answer the store could give, with every network up or down. */
#[test]
fn only_the_choice_itself_leads_anywhere_but_the_mixnet() {
    let defaults = core::iter::once(None).chain((0..=u8::MAX).map(Some));
    for default in defaults {
        for (nym, anon) in [(0, 0), (SOCKS, 0), (0, ANON), (SOCKS, ANON)] {
            let got = pick(default, nym, anon);
            match got {
                Route::Direct => assert_eq!(default, Some(DIRECT)),
                Route::Anon(port) => assert_eq!((default, port), (Some(ANYONE), ANON)),
                Route::Nym(port) => {
                    assert!(default != Some(DIRECT) && default != Some(ANYONE));
                    assert_eq!(port, SOCKS);
                }
                Route::Down(_) => {
                    let up = match default {
                        Some(DIRECT) => true,
                        Some(ANYONE) => anon != 0,
                        _ => nym != 0,
                    };
                    assert!(!up, "{default:?} {nym} {anon}");
                }
            }
        }
    }
}
