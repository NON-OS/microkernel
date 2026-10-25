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

//! What a volume change came to, as the toast says it.

use nonos_audio_proto::{output_short, MasterVolume, VOLUME_MAX};

use super::line::Line;

/// What a volume change came to, as the notice says it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heard {
    /// The service took it and this is what plays.
    Playing(MasterVolume),
    /// Nothing plays. The audio service's output code says why, or None when
    /// the service itself did not answer.
    NoOutput(Option<u32>),
}

const VOLUME: &[u8] = b"Volume ";
const MUTED: &[u8] = b"Muted";
const NO_OUTPUT: &[u8] = b"No sound output";

/// The notice for `heard`, in `out`: "Volume 45%", "Muted", or "No sound
/// output" with the reason in a few words. Returns the bytes written; `out`
/// holds a toast's worth (`TOAST_TEXT_MAX`), which every line here fits.
pub fn notice(heard: Heard, out: &mut [u8]) -> usize {
    let mut line = Line { out, n: 0 };
    match heard {
        Heard::Playing(v) if v.muted => line.put(MUTED),
        Heard::Playing(v) => {
            line.put(VOLUME);
            line.number(v.level.min(VOLUME_MAX));
            line.put(b"%");
        }
        Heard::NoOutput(None) => line.put(NO_OUTPUT),
        Heard::NoOutput(Some(code)) => {
            line.put(NO_OUTPUT);
            line.put(b": ");
            line.put(output_short(code).as_bytes());
        }
    }
    line.n
}

/// Whether a toast's `text` is a volume notice, which the next one replaces
/// rather than stacks under: a held key would otherwise fill the panel with
/// the levels it passed through.
pub fn is_volume_notice(text: &[u8]) -> bool {
    text.starts_with(VOLUME) || text == MUTED || text.starts_with(NO_OUTPUT)
}
