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

//! The route step on a selector with input amps, past the sixteenth input.
//!
//! A selector passes one input, chosen by SET_CONNECT_SEL's eight bits, and
//! the route opens that input's amp with SET_AMP_GAIN_MUTE, whose index has
//! four. The search never routes through an input the amp verb cannot name
//! (`hop`); the step does not rely on it, and for such an input selects it
//! and sends no amp verb, which would land on input `sel - 16` instead.

use crate::constants::{VERB_SET_AMP_GAIN_MUTE, VERB_SET_CONNECT_SEL, WCAP_IN_AMP};
use crate::controller::codec::route::route;
use crate::controller::codec::widget::Widget;
use crate::proofs::amp_index_tests::through;
use crate::proofs::fixtures::{selector, sim_codec};
use crate::sim;

fn amp_selector(nid: u8, conn: &[u8]) -> Widget {
    let mut s = selector(nid, conn);
    s.wcaps |= WCAP_IN_AMP;
    s.amp_in = 0x8000_0000;
    s
}

/// The verbs the route step sends for input `sel` of a 24-input selector.
fn routed(sel: u8) -> Vec<u32> {
    let d = through(amp_selector, 24, 0);
    let s = sim::start(vec![(0, sim_codec(&d))]);
    route(&mut s.link(), 0, &d.widgets[1], sel).expect("the route step failed");
    s.sent()
}

/// The input amp indices the verbs in `sent` address.
fn amps(sent: &[u32]) -> Vec<u32> {
    let amp = |v: &&u32| *v >> 16 & 0xf == VERB_SET_AMP_GAIN_MUTE as u32;
    sent.iter().filter(amp).map(|v| v >> 8 & 0xf).collect()
}

#[test]
fn an_input_past_the_sixteenth_is_selected_and_no_amp_verb_is_sent_for_it() {
    for sel in [16u8, 17, 23, 0xff] {
        let sent = routed(sel);
        let select =
            |v: &u32| v >> 8 & 0xfff == VERB_SET_CONNECT_SEL as u32 && v & 0xff == sel as u32;
        assert!(sent.iter().any(select), "input {sel} was not selected: {sent:x?}");
        assert_eq!(amps(&sent), [], "an amp verb was sent for input {sel}: {sent:x?}");
    }
}

#[test]
fn input_15_is_selected_and_its_own_amp_opened() {
    assert_eq!(amps(&routed(15)), [15]);
}
