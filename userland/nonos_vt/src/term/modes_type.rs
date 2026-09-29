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

//! The modes a program sets, which change how its output is drawn and
//! how keys and the pointer are reported to it.

use super::mouse_mode::MouseMode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Modes {
    /// DECCKM: cursor keys send SS3 rather than CSI.
    pub cursor_keys: bool,
    /// DECKPAM: the keypad sends application sequences.
    pub keypad: bool,
    /// IRM: printing inserts rather than overwrites.
    pub insert: bool,
    /// LNM: line feed also returns the carriage, and Enter sends CR LF.
    pub newline: bool,
    /// DECOM: cursor positions count from the scroll region.
    pub origin: bool,
    /// DECAWM: printing past the last column wraps.
    pub autowrap: bool,
    /// DECSCNM: the whole screen is drawn inverted.
    pub reverse_video: bool,
    /// DECTCEM.
    pub cursor_visible: bool,
    pub cursor_blink: bool,
    pub mouse: MouseMode,
    /// ?1006: mouse reports in the SGR form, which has no coordinate limit.
    pub mouse_sgr: bool,
    /// ?1004: focus changes are reported.
    pub focus: bool,
    /// ?2004: pasted text is bracketed so a shell does not run it.
    pub bracketed_paste: bool,
    /// ?2026: the program is mid-frame; the host may hold the redraw.
    pub sync: bool,
    /// ?1007: the wheel sends cursor keys on the alternate screen.
    pub alt_scroll: bool,
}

impl Default for Modes {
    fn default() -> Modes {
        Modes {
            cursor_keys: false,
            keypad: false,
            insert: false,
            newline: false,
            origin: false,
            autowrap: true,
            reverse_video: false,
            cursor_visible: true,
            cursor_blink: true,
            mouse: MouseMode::Off,
            mouse_sgr: false,
            focus: false,
            bracketed_paste: false,
            sync: false,
            alt_scroll: true,
        }
    }
}
