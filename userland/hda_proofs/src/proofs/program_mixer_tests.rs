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

//! The widgets on either side of a path, read back off the ring: a mixer
//! between DAC and pin opens the DAC's input and closes the loopback, and a
//! beep generator on no path is switched off and muted.

use crate::constants::{AMP_IN_UNMUTE, AMP_MUTE, AMP_OUT_UNMUTE, VERB_SET_BEEP_CONTROL};
use crate::proofs::fixtures::{alc236_hp, alc269_mixers, beep};
use crate::proofs::program_sim::{amp, at, programmed, short};

#[test]
fn a_beep_generator_is_switched_off_and_muted() {
    let mut d = alc236_hp();
    d.widgets.push(beep(0x0f));
    d.widgets.sort_by_key(|w| w.nid);
    let (_s, sent) = programmed(&d, false);
    at(&sent, short(0x0f, VERB_SET_BEEP_CONTROL, 0));
    at(&sent, amp(0x0f, AMP_OUT_UNMUTE | AMP_MUTE));
}

#[test]
fn a_mixer_on_the_path_opens_the_dac_input_and_closes_the_loopback() {
    let (_s, sent) = programmed(&alc269_mixers(), false);
    at(&sent, amp(0x0c, AMP_IN_UNMUTE));
    at(&sent, amp(0x0c, AMP_IN_UNMUTE | (1 << 8) | AMP_MUTE));
}
