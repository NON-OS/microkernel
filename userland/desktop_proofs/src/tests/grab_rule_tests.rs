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

use crate::grab_rule::{
    after_release, after_request, steps, wanted, Answer, Held, Modal, KEY_DOWN_BIT, POINTER_BITS,
};

const fn sure(mask: u32) -> Held {
    Held { mask, unsure: false }
}

const fn unsure(mask: u32) -> Held {
    Held { mask, unsure: true }
}

fn with(f: impl FnOnce(&mut Modal)) -> u32 {
    let mut m = Modal::default();
    f(&mut m);
    wanted(m)
}

#[test]
fn every_thing_drawn_over_the_windows_takes_the_pointer_and_what_answers_keys_takes_them() {
    assert_eq!(with(|_| {}), 0, "a bare desktop holds nothing");
    assert_eq!(with(|m| m.menu = true), POINTER_BITS | KEY_DOWN_BIT, "Esc closes a menu");
    assert_eq!(with(|m| m.dialog = true), POINTER_BITS | KEY_DOWN_BIT, "Tab, Enter, Esc answer it");
    assert_eq!(with(|m| m.drag = true), POINTER_BITS);
    assert_eq!(with(|m| m.launchpad = true), POINTER_BITS | KEY_DOWN_BIT);
    assert_eq!(with(|m| m.rename = true), KEY_DOWN_BIT, "a rename edits, the pointer stays free");
}

#[test]
fn the_pointer_grab_carries_presses_releases_and_absolute_motion() {
    for kind in [3u32, 4, 5, 6, 7] {
        assert_ne!(POINTER_BITS & (1 << kind), 0, "kind {kind}");
    }
    assert_eq!(POINTER_BITS & (1 << 2), 0, "relative motion is left to the router");
    assert_eq!(POINTER_BITS & KEY_DOWN_BIT, 0);
}

#[test]
fn moving_between_holdings_releases_only_when_something_is_dropped() {
    assert_eq!(steps(sure(0), 0), (false, None));
    assert_eq!(steps(sure(0), POINTER_BITS), (false, Some(POINTER_BITS)));
    assert_eq!(
        steps(sure(POINTER_BITS), POINTER_BITS),
        (false, None),
        "no call when nothing changes"
    );
    assert_eq!(steps(sure(POINTER_BITS), 0), (true, None));
    // A rename under an open menu: adding keys is one request.
    let both = POINTER_BITS | KEY_DOWN_BIT;
    assert_eq!(steps(sure(POINTER_BITS), both), (false, Some(both)));
    // The menu closes while the rename goes on: the router drops both grabs
    // on a release, so the keys are asked for again after it.
    assert_eq!(steps(sure(both), KEY_DOWN_BIT), (true, Some(KEY_DOWN_BIT)));
}

/// A release the router did not answer leaves the grab on record, so it is
/// released again; before, the shell forgot it and the router went on
/// sending every key to the shell, the Terminal open in front getting none.
#[test]
fn a_release_with_no_answer_is_released_again() {
    let both = POINTER_BITS | KEY_DOWN_BIT;
    for answer in [Answer::Silent, Answer::Refused] {
        let held = after_release(sure(both), answer);
        assert_eq!(held, sure(both), "{answer:?}");
        assert_eq!(steps(held, 0), (true, None), "{answer:?}: released on the next pass");
    }
    assert_eq!(after_release(sure(both), Answer::Done), sure(0));
    assert_eq!(after_release(unsure(both), Answer::Done), sure(0));
    assert_eq!(steps(after_release(sure(both), Answer::Done), 0), (false, None));
}

/// A request with no answer may have been taken: it is counted held, so the
/// menu's close releases it, and asked for again while it is wanted.
#[test]
fn a_request_with_no_answer_counts_as_held_and_is_asked_again() {
    let both = POINTER_BITS | KEY_DOWN_BIT;
    let held = after_request(sure(0), both, Answer::Silent);
    assert_eq!(held, unsure(both));
    assert_eq!(steps(held, both), (false, Some(both)), "asked again while the menu is up");
    assert_eq!(steps(held, 0), (true, None), "released once the menu has closed");
    assert_eq!(steps(held, KEY_DOWN_BIT), (true, Some(KEY_DOWN_BIT)));
    assert_eq!(after_request(held, both, Answer::Done), sure(both));
    assert_eq!(steps(after_request(held, both, Answer::Done), both), (false, None));
}

#[test]
fn a_refused_request_takes_nothing_new() {
    let both = POINTER_BITS | KEY_DOWN_BIT;
    assert_eq!(after_request(sure(0), both, Answer::Refused), sure(0));
    assert_eq!(after_request(sure(POINTER_BITS), both, Answer::Refused), sure(POINTER_BITS));
    assert_eq!(after_request(sure(0), both, Answer::Done), sure(both));
}

/// Over every holding, wanted mask and answer: a mask the shell no longer
/// records is one the router was told to let go, and a sync with nothing
/// unsure and nothing changed makes no call.
#[test]
fn no_holding_is_forgotten_without_an_answered_release() {
    let masks = [0, KEY_DOWN_BIT, POINTER_BITS, POINTER_BITS | KEY_DOWN_BIT];
    let answers = [Answer::Done, Answer::Refused, Answer::Silent];
    for held in masks.iter().flat_map(|&m| [sure(m), unsure(m)]) {
        for &want in &masks {
            let (release, request) = steps(held, want);
            if held.mask == want && !held.unsure {
                assert_eq!((release, request), (false, None));
            }
            if held.mask & !want != 0 {
                assert!(release, "{held:?} to {want:#x} drops a kind");
            }
            for &a in &answers {
                let after = after_release(held, a);
                if a != Answer::Done {
                    assert_eq!(after, held, "{held:?} {a:?}");
                }
                if let Some(mask) = request {
                    let got = after_request(after, mask, a);
                    if a == Answer::Silent {
                        assert!(got.unsure && got.mask == mask);
                    }
                }
            }
        }
    }
}

#[test]
fn the_sync_records_every_answer_and_stops_while_a_release_is_unanswered() {
    let sync = include_str!("../../../capsule_desktop_shell/src/server/grabs.rs");
    assert!(sync.contains("ctx.links.grab_held = after_release(ctx.links.grab_held, answer);"));
    assert!(sync.contains("if ctx.links.grab_held.mask != 0 {\n            return;"));
    assert!(
        sync.contains("ctx.links.grab_held = after_request(ctx.links.grab_held, mask, answer);")
    );
    assert!(!sync.contains("let _ = crate::input_router_client::release_grab"));
    let call = include_str!("../../../capsule_desktop_shell/src/input_router_client/grab.rs");
    assert!(call.contains("return Answer::Silent;"), "a short reply is no answer");
}
