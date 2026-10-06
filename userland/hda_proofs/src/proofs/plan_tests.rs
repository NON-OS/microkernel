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
//! Choosing outputs and the paths to them on real codec layouts.
//!
//! The driver used to take the first DAC and the first pin it met. On the
//! HP 15s's ALC236 the first pin is the digital microphone at 0x12, so the
//! "output path" was a microphone and the speaker at 0x14 was never touched.

use crate::controller::codec::path::find;
use crate::controller::codec::pincfg::OutKind;
use crate::controller::codec::plan::{plan, Class};
use crate::proofs::fixtures::{
    alc236_hp, alc269_mixers, codec, dac, intel_hdmi, pin, qemu_duplex, selector, Desc,
    PIN_OUT_WCAPS,
};

fn analog(d: &Desc) -> crate::controller::codec::plan::Plan {
    match plan(&codec(d, 0)) {
        Class::Analog(p) => p,
        other => panic!("no analog plan: {other:?}"),
    }
}

#[test]
fn the_hp_15s_codec_drives_its_speaker_and_its_headphone_jack() {
    let p = analog(&alc236_hp());
    let outs: Vec<_> = p.iter().map(|o| (o.kind, o.pin(), o.path.dac())).collect();
    assert_eq!(outs, vec![(OutKind::Speaker, 0x14, 0x02), (OutKind::Headphone, 0x21, 0x02)]);
    assert_eq!(p.sensing_headphone(), Some(0x21), "the headphone jack can be sensed");
    assert!(p.iter().all(|o| o.pin() != 0x12 && o.pin() != 0x1e), "a microphone or S/PDIF pin was driven");
}

#[test]
fn pins_marked_not_connected_are_skipped_even_when_they_could_output() {
    let p = analog(&alc236_hp());
    assert!(p.iter().all(|o| o.pin() != 0x1a && o.pin() != 0x1b));
}

#[test]
fn a_display_codec_is_recognised_as_digital_only() {
    assert_eq!(plan(&codec(&intel_hdmi(), 2)), Class::DigitalOnly);
}

#[test]
fn qemus_codec_still_plays_through_its_line_out() {
    let p = analog(&qemu_duplex());
    let outs: Vec<_> = p.iter().map(|o| (o.kind, o.pin(), o.path.dac())).collect();
    assert_eq!(outs, vec![(OutKind::LineOut, 0x03, 0x02)]);
}

#[test]
fn a_path_through_a_mixer_records_which_mixer_input_the_dac_is_on() {
    let p = analog(&alc269_mixers());
    let spk = p.iter().find(|o| o.kind == OutKind::Speaker).unwrap();
    assert_eq!(&spk.path.nodes[..3], &[0x14, 0x0c, 0x02]);
    assert_eq!(&spk.path.sel[..2], &[0, 0], "the DAC is input 0 of mixer 0x0c");
}

#[test]
fn a_dac_that_cannot_play_48k_16_bit_is_passed_over() {
    let mut d = alc236_hp();
    d.widgets[0] = dac(0x02, 0);
    let p = analog(&d);
    for o in p.iter() {
        assert_eq!(o.path.dac(), 0x03);
        assert_eq!(o.path.sel[0], 1, "the pin must select its second input");
    }
}

#[test]
fn two_outputs_sharing_a_selector_agree_on_its_input() {
    let d = Desc {
        vendor: 0x10ec_0662,
        subsystem: 0,
        gpio: 0,
        widgets: vec![
            dac(0x02, 0x0002_0040),
            dac(0x03, 0x0002_0040),
            selector(0x0f, &[0x02, 0x03]),
            pin(0x14, PIN_OUT_WCAPS, 0x14, 0x9017_0110, &[0x0f]),
            pin(0x15, PIN_OUT_WCAPS, 0x1c, 0x0221_1020, &[0x0f]),
        ],
    };
    let c = codec(&d, 0);
    let spk = find(&c, 0x14, &[]).unwrap();
    let hp = find(&c, 0x15, &[spk]).unwrap();
    assert_eq!((spk.sel[1], hp.sel[1]), (0, 0), "the shared selector was asked for two inputs");
}

#[test]
fn a_codec_with_no_routable_output_says_so() {
    let mut d = alc236_hp();
    d.widgets.retain(|w| w.nid != 0x02 && w.nid != 0x03);
    assert_eq!(plan(&codec(&d, 0)), Class::NoOutput);
}
