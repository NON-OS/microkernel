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
//! Intel SST engines are named, never run as HD Audio controllers.
//!
//! A Broadwell ULT laptop with its DSP on shows the ADSP (8086:9cb6) and
//! the display's HDMI controller (8086:160c); a Cherry Trail tablet shows
//! only the LPE engine (8086:22a8). The ADSP's BAR0 is no HD Audio register
//! set, so bringing it up as a controller cannot succeed and the driver
//! would give up; the HDMI controller alone said "HDMI only". Both must read "needs
//! SOF", and the plain HD Audio controllers of the same era must not.

use crate::controller::intel::hda_controller;
use crate::controller::sst::intel_sst;
use crate::controller::verdict::{combine, Verdict};
use crate::controller::verdict_name::name;

#[test]
fn every_sst_engine_is_known_by_its_id() {
    for dev in [0x9c36, 0x9cb6, 0x0f28, 0x22a8, 0x119a] {
        assert!(intel_sst(0x8086, dev), "8086:{dev:04x} was not taken for an SST engine");
    }
}

#[test]
fn hd_audio_controllers_of_the_same_era_are_not_sst() {
    // Lynx Point-LP and Wildcat Point-LP HDA, Broadwell HDMI, Braswell HDA,
    // Bay Trail HDA, and a Skylake controller with its DSP on.
    for dev in [0x9c20, 0x9ca0, 0x160c, 0x2284, 0x0f04, 0x9d70, 0x02c8] {
        assert!(!intel_sst(0x8086, dev), "8086:{dev:04x} was taken for an SST engine");
    }
    assert!(!intel_sst(0x1022, 0x9cb6), "an id is Intel's only with Intel's vendor");
}

#[test]
fn an_sst_engine_beside_an_hdmi_controller_reads_needs_sof() {
    // setup::run starts from the bus verdict and folds each controller in.
    let mut seen = Some(Verdict::NeedsSof);
    for v in [Verdict::HdmiOnly, Verdict::NoCodec, Verdict::NoOutputPath] {
        seen = Some(seen.map_or(v, |w| combine(w, v)));
    }
    assert_eq!(seen, Some(Verdict::NeedsSof));
}

#[test]
fn the_class_test_alone_would_have_taken_a_class_0401_engine() {
    assert!(hda_controller(0x8086, 0x01));
}

#[test]
fn every_verdict_has_words_for_the_log() {
    assert_eq!(name(Verdict::NeedsSof as u32), "needs Intel SOF, the speakers are behind the DSP");
    for v in [0u32, 2, 3, 4, 5, 6] {
        assert_ne!(name(v), "unknown", "verdict {v} has no words");
        assert!(name(v).len() <= 60, "verdict {v} would not fit the 120-byte log line");
    }
    assert_eq!(name(1), "unknown");
}
