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

//! Input amps past the sixteenth input, on the verbs' side.
//!
//! The driver used to unmute the chosen mixer input and mute every other,
//! all through SET_AMP_GAIN_MUTE's four-bit index. With the DAC at input 17
//! the unmute wrapped to input 1, a microphone loopback, and played it to
//! the speaker; with it at input 1, the mute sent for input 17 silenced it.

use crate::constants::{AMP_MUTE, AMP_SET_INPUT, VERB_SET_AMP_GAIN_MUTE};
use crate::controller::codec::path::{Path, MAX_DEPTH};
use crate::controller::codec::pincfg::OutKind;
use crate::controller::codec::plan::{Output, Plan, MAX_OUTPUTS};
use crate::controller::codec::program::program;
use crate::proofs::amp_index_tests::through;
use crate::proofs::fixtures::{codec, mixer, sim_codec};
use crate::sim;

/// Program a speaker path through mixer input `sel` of `n`; return the
/// mixer's input amp verbs in order, as (index, muted).
fn mixer_amps(n: usize, sel: u8) -> Vec<(u32, bool)> {
    let d = through(mixer, n, sel as usize);
    let s = sim::start(vec![(0, sim_codec(&d))]);
    let mut path = Path { nodes: [0; MAX_DEPTH], sel: [0; MAX_DEPTH], len: 3 };
    path.nodes[..3].copy_from_slice(&[0x14, 0x0c, 0x02]);
    path.sel[1] = sel;
    let mut plan = Plan { outputs: [None; MAX_OUTPUTS] };
    plan.outputs[0] = Some(Output { kind: OutKind::Speaker, path, senses: false });
    program(&mut s.link(), &codec(&d, 0), &plan, 1).expect("programming failed");
    let to_mixer =
        |v: u32| v >> 20 & 0x7f == 0x0c && v >> 16 & 0xf == VERB_SET_AMP_GAIN_MUTE as u32;
    let sent = s.sent().into_iter().filter(|&v| to_mixer(v) && v & AMP_SET_INPUT as u32 != 0);
    sent.map(|v| (v >> 8 & 0xf, v & AMP_MUTE as u32 != 0)).collect()
}

/// The state each input amp the verbs addressed was left in.
fn last(amps: &[(u32, bool)], index: u32) -> Option<bool> {
    amps.iter().rev().find(|a| a.0 == index).map(|a| a.1)
}

#[test]
fn the_dac_at_input_17_never_opens_the_microphone_at_input_1() {
    let amps = mixer_amps(20, 17);
    assert_eq!(last(&amps, 1), Some(true), "the microphone loopback was left open: {amps:?}");
}

#[test]
fn the_dac_at_input_1_is_not_muted_again_by_input_17() {
    let amps = mixer_amps(20, 1);
    assert_eq!(last(&amps, 1), Some(false), "the chosen input was muted after: {amps:?}");
    let mut seen: Vec<u32> = amps.iter().map(|a| a.0).collect();
    seen.dedup();
    assert_eq!(seen, (0..16).collect::<Vec<_>>(), "an input amp was addressed twice");
}
