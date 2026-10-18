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

//! A card that fails is never left as it was (C1): a request that fails on
//! the card puts it to be identified again before the next, and a first
//! card that will not come up is left powered down, not retried at every
//! look, until it is taken out.

use super::rig::{knobs, sdhc, serving, up};
use super::service_rig::{call, span, KERNEL};
use crate::model::{with, Knobs};
use crate::protocol::*;
use crate::serve::medium::Medium;

#[test]
fn a_request_that_fails_on_the_card_has_it_identified_again_before_the_next() {
    let (mut drv, mut medium) = serving(sdhc(1));
    knobs(Knobs { stall_data: true, ..Knobs::default() });
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 2)).status, E_TIMEDOUT);
    assert!(matches!(medium, Medium::Check(_)), "{medium:?}");
    knobs(Knobs::default());
    with(|r| r.sent.clear());
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 2)).status, E_OK);
    with(|r| assert!(r.sent.starts_with(&[0, 8]), "not identified again: {:?}", r.sent));
}

#[test]
fn a_first_card_that_will_not_come_up_waits_powered_down_until_it_is_out() {
    let mut drv = up(None);
    let mut medium = Medium::Empty;
    let mut mute = sdhc(1);
    mute.busy_polls = u32::MAX;
    with(|r| r.insert(mute));
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 1)).status, E_NOMEDIUM);
    assert_eq!(medium, Medium::Refused);
    with(|r| (r.sent.clear(), assert!(!r.powered())));
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 1)).status, E_NOMEDIUM);
    with(|r| assert!(r.sent.is_empty(), "tried again: {:?}", r.sent));
    with(|r| r.pull().map(|_| r.insert(sdhc(2))));
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 1)).status, E_OK);
}
