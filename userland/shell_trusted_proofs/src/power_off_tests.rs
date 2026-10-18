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

//! A power off is sent only on the router's Power Off to a confirmation that
//! is up, and once per confirmation. Every sequence of asks and answers up
//! to ten steps long is played against a model of that rule.

use crate::state::input_source_rule::{Routed, Router};
use crate::state::power_off::{PowerOff, POWER_OFF_FAILED};
use crate::state::toast::TOAST_TEXT_MAX;

fn by() -> Routed {
    Router::Pinned(7).frame(7).expect("the router's own frame")
}

#[test]
fn every_sequence_sends_only_on_a_confirmation_that_was_up() {
    for len in 0..=10u32 {
        for code in 0..3usize.pow(len) {
            let (mut p, mut up, mut asks, mut sends, mut c) =
                (PowerOff::default(), false, 0, 0, code);
            for _ in 0..len {
                let step = c % 3;
                c /= 3;
                if step == 0 {
                    p.ask();
                    (up, asks) = (true, asks + 1);
                } else {
                    let confirm = step == 2;
                    let sent = p.answer(by(), confirm).is_some();
                    assert_eq!(sent, up && confirm, "sequence {code} of {len}");
                    sends += usize::from(sent);
                    up = false;
                }
                assert_eq!(p.showing(), up, "sequence {code} of {len}");
            }
            assert!(sends <= asks, "more sends than confirmations asked for");
        }
    }
}

#[test]
fn nothing_is_sent_before_an_ask_or_on_cancel() {
    let mut p = PowerOff::default();
    assert!(!p.showing());
    assert!(p.answer(by(), true).is_none(), "Power Off with nothing asked");
    p.ask();
    p.ask();
    assert!(p.showing(), "a second ask keeps the one confirmation");
    assert!(p.answer(by(), false).is_none(), "Cancel, Esc or a press outside");
    assert!(!p.showing());
    assert!(p.answer(by(), true).is_none(), "a late Power Off after Cancel");
}

#[test]
fn the_failure_fits_one_toast() {
    assert!(POWER_OFF_FAILED.len() <= TOAST_TEXT_MAX, "cut in a toast");
    assert!(POWER_OFF_FAILED.is_ascii());
}
