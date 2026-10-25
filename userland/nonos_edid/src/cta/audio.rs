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

//! Short audio descriptors, three bytes each in the audio data block,
//! tag 1 (CTA-861-H section 7.5.2 and its short audio descriptor tables).

use super::blocks::Cta;
use crate::edid::Edid;

pub const TAG_AUDIO: u8 = 1;

/// One format the sink plays.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShortAudio {
    /// Byte 1 bits 6-3: 1 is L-PCM; the rest name compressed formats.
    pub format: u8,
    /// Byte 1 bits 2-0 hold the channels less one.
    pub channels: u8,
    /// Byte 2: bit 0 32 kHz, 1 44.1, 2 48, 3 88.2, 4 96, 5 176.4, 6 192.
    pub rates: u8,
    /// Byte 3: the sample sizes for L-PCM, the format's own data else.
    pub detail: u8,
}

impl Edid<'_> {
    /// Every short audio descriptor in every CTA extension.
    pub fn for_each_audio(&self, f: &mut dyn FnMut(ShortAudio)) {
        for c in self.extensions().filter_map(Cta::new) {
            for b in c.data_blocks().filter(|b| b.tag == TAG_AUDIO) {
                for s in b.body.chunks_exact(3) {
                    let format = (s[0] >> 3) & 0x0F;
                    if format != 0 {
                        let channels = (s[0] & 7) + 1;
                        f(ShortAudio { format, channels, rates: s[1] & 0x7F, detail: s[2] });
                    }
                }
            }
        }
    }

    /// Any CTA extension claims basic audio (two channel L-PCM).
    pub fn basic_audio(&self) -> bool {
        self.extensions().filter_map(Cta::new).any(|c| c.basic_audio())
    }
}
