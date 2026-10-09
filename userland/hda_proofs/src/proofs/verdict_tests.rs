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
//! Naming the machine's audio when it cannot play.
//!
//! The target machine is a Gemini Lake laptop (8086:3198). Linux's
//! intel-dsp-config hands Gemini Lake to SOF only on Chromebooks and ES8336
//! boards; everywhere else, and always when a codec answers, it is plain HD
//! Audio, and must never be called SOF-only.

use crate::controller::intel::{amd_acp, dsp_capable, graphics_audio, hda_controller, skl_family};
use crate::controller::verdict::{combine, judge, Findings, Verdict};

const GLK: (u16, u16) = (0x8086, 0x3198);

fn f(mask: u16, analog: bool, digital_only: bool) -> Findings {
    Findings { codec_mask: mask, analog, digital_only }
}

#[test]
fn gemini_lake_with_an_answering_analog_codec_plays() {
    let dsp = dsp_capable(GLK.0, GLK.1, 0x03);
    assert!(dsp);
    assert_eq!(judge(dsp, f(0b101, true, false)), Verdict::Ready);
}

#[test]
fn a_dsp_controller_with_no_codec_or_only_hdmi_needs_sof() {
    assert_eq!(judge(true, f(0, false, false)), Verdict::NeedsSof);
    assert_eq!(judge(true, f(0b100, false, true)), Verdict::NeedsSof);
}

#[test]
fn a_plain_controller_with_no_codec_or_only_hdmi_is_named_for_that() {
    assert_eq!(judge(false, f(0, false, false)), Verdict::NoCodec);
    assert_eq!(judge(false, f(1, false, true)), Verdict::HdmiOnly);
    assert_eq!(judge(false, f(1, false, false)), Verdict::NoOutputPath);
}

#[test]
fn the_controller_that_explains_the_machine_best_names_it() {
    // A laptop whose own controller needs SOF, beside a graphics card with HDMI.
    assert_eq!(combine(Verdict::HdmiOnly, Verdict::NeedsSof), Verdict::NeedsSof);
    // An AMD laptop whose HD Audio carries only HDMI and whose sound is on the ACP.
    assert_eq!(combine(Verdict::HdmiOnly, Verdict::AmdAcp), Verdict::AmdAcp);
    assert_eq!(combine(Verdict::Ready, Verdict::AmdAcp), Verdict::Ready);
}

#[test]
fn the_wire_numbers_are_fixed() {
    let all = [
        (Verdict::Ready, 0),
        (Verdict::NeedsSof, 2),
        (Verdict::NoCodec, 3),
        (Verdict::HdmiOnly, 4),
        (Verdict::NoOutputPath, 5),
        (Verdict::AmdAcp, 6),
    ];
    for (v, n) in all {
        assert_eq!(v as u32, n);
    }
}

#[test]
fn class_0401_intel_controllers_are_hd_audio_and_other_vendors_0401_are_not() {
    assert!(hda_controller(0x8086, 0x01), "a Skylake+ controller with its DSP enabled was skipped");
    assert!(hda_controller(0x1022, 0x03));
    assert!(!hda_controller(0x1022, 0x01));
    assert!(!hda_controller(0x8086, 0x80));
}

#[test]
fn the_vendor_tables_hold() {
    assert!(skl_family(0x8086, 0x3198) && skl_family(0x8086, 0x5a98) && skl_family(0x8086, 0x51c8));
    assert!(!skl_family(0x8086, 0x293e), "QEMU's ICH9 has no DSP");
    assert!(dsp_capable(0x8086, 0x0000, 0x01), "class 0x0401 marks a DSP whatever the id");
    assert!(!dsp_capable(0x1022, 0x15e3, 0x03));
    assert!(graphics_audio(0x1002, 0xab38) && graphics_audio(0x10de, 0x10f0));
    assert!(graphics_audio(0x8086, 0x4f90), "Intel Arc carries HDMI only");
    assert!(!graphics_audio(0x1022, 0x15e3));
    assert!(amd_acp(0x1022, 0x04, 0x80));
    assert!(!amd_acp(0x1022, 0x04, 0x03));
}
