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
//! What this machine's audio is, in the words the user will read.
//!
//! A controller whose codecs give an analog output plays. Otherwise the
//! reason has to be named, not left as silence. Intel controllers from
//! Skylake on carry an audio DSP; on laptops whose speakers and microphones
//! hang off it over I2S or SoundWire there is no HD Audio codec on the link
//! at all, or only the display's HDMI codec, and Linux hands those machines
//! to Sound Open Firmware (`snd_intel_dsp_driver_probe`,
//! sound/hda/intel-dsp-config.c). A machine whose analog codec answers plays
//! through it here whatever intel-dsp-config would pick for its microphones:
//! playback needs only the codec. AMD laptops that route audio through the
//! ACP coprocessor (PCI 1022:15e2 and its kin, class 0x0480) are the same
//! case on the other vendor.
//!
//! The numbers are the wire values of `OP_OUTPUT_STATUS`; the audio server
//! passes them on unchanged (`nonos_audio_proto::output`).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    Ready = 0,
    NeedsSof = 2,
    NoCodec = 3,
    HdmiOnly = 4,
    NoOutputPath = 5,
    AmdAcp = 6,
}

/// What one controller's codecs added up to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Findings {
    pub codec_mask: u16,
    pub analog: bool,
    pub digital_only: bool,
}

/// One controller's verdict. `dsp_capable` is a controller that can route
/// its audio through a DSP instead of an HD Audio codec (`intel::dsp_capable`).
pub const fn judge(dsp_capable: bool, f: Findings) -> Verdict {
    if f.analog {
        return Verdict::Ready;
    }
    if f.codec_mask == 0 {
        return if dsp_capable { Verdict::NeedsSof } else { Verdict::NoCodec };
    }
    if f.digital_only {
        return if dsp_capable { Verdict::NeedsSof } else { Verdict::HdmiOnly };
    }
    Verdict::NoOutputPath
}

/// How much a verdict tells the user, so the machine is described by the
/// controller that explains it best: a graphics card's HDMI-only controller
/// must not hide the reason the main one is silent.
const fn weight(v: Verdict) -> u8 {
    match v {
        Verdict::Ready => 6,
        Verdict::NeedsSof => 5,
        Verdict::AmdAcp => 4,
        Verdict::NoOutputPath => 3,
        Verdict::HdmiOnly => 2,
        Verdict::NoCodec => 1,
    }
}

/// The machine's verdict from two. An AMD ACP on the bus explains a machine
/// whose HD Audio controllers found no analog output.
pub const fn combine(a: Verdict, b: Verdict) -> Verdict {
    if weight(a) >= weight(b) {
        a
    } else {
        b
    }
}
