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

use crate::controller::codec::pincfg::OutKind;
use crate::controller::codec::plan::{plan, Class};
use crate::proofs::fixtures::{alc236_hp, codec, intel_hdmi, qemu_duplex, Desc};

pub fn analog(d: &Desc) -> crate::controller::codec::plan::Plan {
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
    assert!(
        p.iter().all(|o| o.pin() != 0x12 && o.pin() != 0x1e),
        "a microphone or S/PDIF pin was driven"
    );
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
fn a_codec_with_no_routable_output_says_so() {
    let mut d = alc236_hp();
    d.widgets.retain(|w| w.nid != 0x02 && w.nid != 0x03);
    assert_eq!(plan(&codec(&d, 0)), Class::NoOutput);
}
