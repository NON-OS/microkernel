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

//! The verbs that make the HP 15s's ALC236 play, read back off the ring.
//!
//! Every verb is answered whether or not the codec acts on it, so a missing
//! one shows only as silence on the machine. These pin each one Linux sends
//! for this codec: power, the Realtek EAPD coefficient and `alc256_init`,
//! the pin's input selection, each amp on the path unmuted at 0 dB, the
//! pins' output enables, EAPD, the HP GPIOs, and the converter joined to the
//! stream with the format SDnFMT carries.

use crate::constants::{
    PIN_HP_ENABLE, PIN_OUT_ENABLE, POWER_D0, VERB_SET_CONNECT_SEL, VERB_SET_PIN_WIDGET_CONTROL,
    VERB_SET_POWER_STATE,
};
use crate::proofs::fixtures::{alc236_hp, qemu_duplex};
use crate::proofs::program_sim::{at, programmed, short};

#[test]
fn the_function_group_is_powered_before_anything_else_is_set() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    assert_eq!(sent[0], short(1, VERB_SET_POWER_STATE, POWER_D0 as u16));
    let sense = at(&sent, short(0x21, crate::constants::VERB_GET_PIN_SENSE, 0));
    let d0 = at(&sent, short(1, crate::constants::VERB_GET_POWER_STATE, 0));
    assert!(d0 < sense, "the jack was read before the codec reached D0");
}

#[test]
fn the_speaker_and_headphone_pins_select_the_dac_and_are_enabled_for_output() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    at(&sent, short(0x14, VERB_SET_CONNECT_SEL, 0));
    at(&sent, short(0x21, VERB_SET_CONNECT_SEL, 0));
    at(&sent, short(0x14, VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT_ENABLE as u16));
    let hp = (PIN_OUT_ENABLE | PIN_HP_ENABLE) as u16;
    at(&sent, short(0x21, VERB_SET_PIN_WIDGET_CONTROL, hp));
}

#[test]
fn with_headphones_in_the_speaker_pin_is_switched_off() {
    let (_s, sent) = programmed(&alc236_hp(), true);
    at(&sent, short(0x14, VERB_SET_PIN_WIDGET_CONTROL, 0));
    assert!(!sent.contains(&short(0x14, VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT_ENABLE as u16)));
}

#[test]
fn a_codec_that_is_not_realtek_gets_no_coefficient_writes() {
    let (_s, sent) = programmed(&qemu_duplex(), false);
    assert!(sent.iter().all(|c| (c >> 16) & 0xf != 0x5), "a coefficient index was written");
    at(&sent, short(0x03, VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT_ENABLE as u16));
}
