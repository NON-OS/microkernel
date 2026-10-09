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
//! What the audio service said when the Sound page asked whether this
//! machine can play, and how the row reads it. A machine whose sound
//! hardware NONOS cannot drive (an Intel laptop that needs the SOF DSP
//! firmware, an AMD audio coprocessor, HDMI only) is named here in plain
//! words rather than left to read as silence.

use nonos_audio_proto::{
    output_message, output_short, FLAG_HEADPHONE, FLAG_LINE_OUT, FLAG_PLUGGED, FLAG_SPEAKER,
    OUTPUT_READY,
};

use crate::settings::schema::rows::Tone;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AudioOutput {
    /// Not asked yet: the Sound page has not been opened.
    Unasked,
    /// audio.server is not running, so there is nobody to ask.
    NoService,
    /// The service's answer: an output code and its flags.
    Status { code: u32, flags: u32 },
}

const ASKED: &str = "As the sound hardware reported when this page opened";

/// The row's value, its tone, and the line under its label.
pub fn said(o: AudioOutput) -> (&'static str, Tone, &'static str) {
    match o {
        AudioOutput::Unasked => ("--", Tone::Idle, ASKED),
        AudioOutput::NoService => {
            ("Audio service not running", Tone::Warn, "The audio service did not start, so nothing can play")
        }
        AudioOutput::Status { code: OUTPUT_READY, flags } => (playing_through(flags), Tone::Ok, ASKED),
        AudioOutput::Status { code, .. } => (output_short(code), Tone::Warn, output_message(code)),
    }
}

fn playing_through(flags: u32) -> &'static str {
    if flags & FLAG_PLUGGED != 0 {
        "Headphones"
    } else if flags & FLAG_SPEAKER != 0 && flags & FLAG_HEADPHONE != 0 {
        "Speakers and headphone jack"
    } else if flags & FLAG_SPEAKER != 0 {
        "Speakers"
    } else if flags & FLAG_HEADPHONE != 0 {
        "Headphone jack"
    } else if flags & FLAG_LINE_OUT != 0 {
        "Line out"
    } else {
        "Working"
    }
}
