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

use std::sync::atomic::Ordering;

use crate::constants::{
    AMP_IN_UNMUTE, AMP_MUTE, AMP_OUT_UNMUTE, PIN_HP_ENABLE, PIN_OUT_ENABLE, POWER_D0,
    VERB_SET_AMP_GAIN_MUTE, VERB_SET_CHANNEL_STREAMID, VERB_SET_CONNECT_SEL,
    VERB_SET_EAPD_BTLENABLE, VERB_SET_GPIO_DATA, VERB_SET_GPIO_DIRECTION, VERB_SET_GPIO_MASK,
    VERB_SET_BEEP_CONTROL, VERB_SET_PIN_WIDGET_CONTROL, VERB_SET_POWER_STATE,
    VERB_SET_STREAM_FORMAT,
};
use crate::controller::codec::format::PLAYBACK;
use crate::controller::codec::plan::{plan, Class};
use crate::controller::codec::program::{hp_gpio, out_amp, program, set_outputs, zero_db};
use crate::controller::codec::walk::walk;
use crate::controller::{compose_verb, compose_verb_long};
use crate::proofs::fixtures::{alc236_hp, alc269_mixers, beep, qemu_duplex, sim_codec, Desc};
use crate::sim::{self, Sim};

const TAG: u8 = 1;

fn programmed(d: &Desc, plugged: bool) -> (Sim, Vec<u32>) {
    let s = sim::start(vec![(0, sim_codec(d))]);
    s.coefs.lock().unwrap().insert((0, 0x20, 0x10), 0x0220);
    s.plugged.store(plugged, Ordering::Release);
    let mut link = s.link();
    let c = walk(&mut link, 0).unwrap().unwrap();
    let Class::Analog(p) = plan(&c) else { panic!("no plan") };
    let before = s.sent().len();
    let sensed = program(&mut link, &c, &p, TAG).expect("programming failed");
    assert_eq!(sensed, plugged, "the jack was not read");
    let sent = s.sent()[before..].to_vec();
    (s, sent)
}

fn at(sent: &[u32], cmd: u32) -> usize {
    sent.iter().position(|&c| c == cmd).unwrap_or_else(|| panic!("{cmd:#010x} was never sent"))
}

fn short(nid: u8, verb: u16, payload: u16) -> u32 {
    compose_verb(0, nid, verb, payload)
}

fn amp(nid: u8, payload: u16) -> u32 {
    compose_verb_long(0, nid, VERB_SET_AMP_GAIN_MUTE as u16, payload)
}

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
fn every_amp_on_the_path_is_left_muted_at_its_zero_db_step_until_a_stream_plays() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    at(&sent, amp(0x02, AMP_OUT_UNMUTE | AMP_MUTE | 0x57));
    at(&sent, amp(0x14, AMP_OUT_UNMUTE | AMP_MUTE));
    at(&sent, amp(0x21, AMP_OUT_UNMUTE | AMP_MUTE));
    for nid in [0x02u8, 0x14, 0x21] {
        let open = sent.iter().any(|&c| {
            c >> 8 == amp(nid, 0) >> 8 && c & 0x8000 != 0 && c & u32::from(AMP_MUTE) == 0
        });
        assert!(!open, "node {nid:#04x} was opened before any stream ran");
    }
    assert!(!sent.contains(&amp(0x02, AMP_OUT_UNMUTE | 0x7f)), "a gain past the amp's steps");
    assert_eq!(zero_db(0x0005_7057), 0x57);
    assert_eq!(zero_db(0x0000_0060), 0, "an offset past the step count is held to it");
}

#[test]
fn a_stream_opens_every_amp_on_the_path_at_zero_db_and_its_stop_closes_them() {
    let d = alc236_hp();
    let s = sim::start(vec![(0, sim_codec(&d))]);
    let mut link = s.link();
    let c = walk(&mut link, 0).unwrap().unwrap();
    let Class::Analog(p) = plan(&c) else { panic!("no plan") };
    program(&mut link, &c, &p, TAG).unwrap();
    let before = s.sent().len();
    set_outputs(&mut link, &c, &p, true).unwrap();
    let opened = s.sent()[before..].to_vec();
    at(&opened, amp(0x02, AMP_OUT_UNMUTE | 0x57));
    at(&opened, amp(0x14, AMP_OUT_UNMUTE));
    at(&opened, amp(0x21, AMP_OUT_UNMUTE));
    let before = s.sent().len();
    set_outputs(&mut link, &c, &p, false).unwrap();
    let closed = s.sent()[before..].to_vec();
    at(&closed, amp(0x02, AMP_OUT_UNMUTE | AMP_MUTE | 0x57));
    at(&closed, amp(0x14, AMP_OUT_UNMUTE | AMP_MUTE));
    at(&closed, amp(0x21, AMP_OUT_UNMUTE | AMP_MUTE));
    assert_eq!(out_amp(0x0005_7057, true), AMP_OUT_UNMUTE | 0x57);
    assert_eq!(out_amp(0x0005_7057, false), AMP_OUT_UNMUTE | AMP_MUTE | 0x57);
}

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
fn eapd_is_raised_on_every_pin_that_has_it() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    for nid in [0x14u8, 0x1b, 0x21] {
        at(&sent, short(nid, VERB_SET_EAPD_BTLENABLE, 0x02));
    }
}

#[test]
fn the_realtek_eapd_coefficient_and_alc256_init_are_applied() {
    let (s, _sent) = programmed(&alc236_hp(), false);
    let coefs = s.coefs.lock().unwrap();
    assert_eq!(coefs.get(&(0, 0x20, 0x10)), Some(&0x0020), "coef 0x10 bit 9 still set");
    assert_eq!(coefs.get(&(0, 0x20, 0x36)), Some(&0x5757));
    assert_eq!(coefs.get(&(0, 0x57, 0x04)).map(|v| v & 7), Some(4), "headphone amp left in low power");
}

#[test]
fn an_hp_machine_gets_its_gpios_driven_high_as_outputs() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    let m = at(&sent, short(1, VERB_SET_GPIO_MASK, 0x07));
    let d = at(&sent, short(1, VERB_SET_GPIO_DIRECTION, 0x07));
    let v = at(&sent, short(1, VERB_SET_GPIO_DATA, 0x07));
    assert!(m < d && d < v, "mask, direction, then data, as Linux writes them");
    assert_eq!(hp_gpio(0x1025_0000, 3), None, "only HP machines");
    assert_eq!(hp_gpio(0x103c_8651, 0), None);
    assert_eq!(hp_gpio(0x103c_8651, 9), Some(0xff));
}

#[test]
fn the_converter_takes_the_stream_tag_and_the_format_sdnfmt_carries() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    let f = at(&sent, compose_verb_long(0, 0x02, VERB_SET_STREAM_FORMAT as u16, PLAYBACK));
    let t = at(&sent, short(0x02, VERB_SET_CHANNEL_STREAMID, (TAG as u16) << 4));
    assert!(f < t, "the stream joined before the format was set");
    let joins = sent.iter().filter(|&&c| c == short(0x02, VERB_SET_CHANNEL_STREAMID, 0x10)).count();
    assert_eq!(joins, 1, "a DAC shared by two outputs was joined twice");
}

#[test]
fn with_headphones_in_the_speaker_pin_is_switched_off() {
    let (_s, sent) = programmed(&alc236_hp(), true);
    at(&sent, short(0x14, VERB_SET_PIN_WIDGET_CONTROL, 0));
    assert!(!sent.contains(&short(0x14, VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT_ENABLE as u16)));
}

#[test]
fn a_mixer_on_the_path_opens_the_dac_input_and_closes_the_loopback() {
    let (_s, sent) = programmed(&alc269_mixers(), false);
    at(&sent, amp(0x0c, AMP_IN_UNMUTE));
    at(&sent, amp(0x0c, AMP_IN_UNMUTE | (1 << 8) | AMP_MUTE));
}

#[test]
fn a_codec_that_is_not_realtek_gets_no_coefficient_writes() {
    let (_s, sent) = programmed(&qemu_duplex(), false);
    assert!(sent.iter().all(|c| (c >> 16) & 0xf != 0x5), "a coefficient index was written");
    at(&sent, short(0x03, VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT_ENABLE as u16));
}
