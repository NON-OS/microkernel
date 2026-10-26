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
//! Cirrus Logic's init sequences (controller/codec/cirrus) read back off
//! the ring: the CS4206 and CS4207 errata and coefficient verbs, the
//! CS4208's, each whole and in order before the stream is joined, and
//! none for a codec not Cirrus's.

use crate::constants::{VERB_SET_GPIO_DATA, VERB_SET_STREAM_FORMAT};
use crate::controller::codec::cirrus::{Verb, CS4208, CS420X};
use crate::controller::codec::cirrus_verbs::SET_PROC_STATE;
use crate::controller::compose_verb_long;
use crate::proofs::fixtures::{alc236_hp, Desc};
use crate::proofs::program_sim::{at, programmed, short};

/// The ALC236 HP's widgets as a Cirrus codec with four GPIOs in `subsystem`.
fn cirrus(vendor: u32, subsystem: u32) -> Desc {
    Desc { vendor, subsystem, gpio: 4, ..alc236_hp() }
}

fn word(&(nid, verb, v): &Verb) -> u32 {
    if verb <= 0xf {
        compose_verb_long(0, nid, verb, v)
    } else {
        short(nid, verb, v)
    }
}

/// Where each of `verbs` was sent, each after the one before.
fn in_order(sent: &[u32], verbs: &[Verb]) -> usize {
    verbs.iter().fold(0, |from, v| {
        let at = sent[from..].iter().position(|&c| c == word(v));
        from + at.unwrap_or_else(|| panic!("{v:x?} not sent after {from}")) + 1
    })
}

#[test]
fn a_cs4206_gets_its_errata_and_coefficients_in_order_then_its_mac_gpios() {
    let (_s, sent) = programmed(&cirrus(0x1013_4206, 0x106b_1c00), false);
    let done = in_order(&sent, &CS420X);
    let stream = sent.iter().position(|c| c >> 16 & 0xf == VERB_SET_STREAM_FORMAT as u32);
    assert!(done <= stream.expect("the stream set"), "the codec set up before the stream");
    at(&sent, short(1, VERB_SET_GPIO_DATA, 0x0a));
}

#[test]
fn a_cs4208_gets_its_own_sequence_and_gpio0() {
    let (_s, sent) = programmed(&cirrus(0x1013_4208, 0x106b_7100), false);
    in_order(&sent, &CS4208);
    assert!(!sent.contains(&word(&CS420X[2])), "no CS4206 errata on a CS4208");
    at(&sent, short(1, VERB_SET_GPIO_DATA, 0x01));
}

#[test]
fn a_codec_not_cirruss_gets_no_processing_state() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    assert!(sent.iter().all(|c| c >> 8 & 0xfff != SET_PROC_STATE as u32), "{sent:x?}");
}
