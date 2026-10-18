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

//! The last raw frame and how often it has repeated verbatim. Polled reads
//! return the current report whether or not it is new; without this, one
//! stale motion report re-read at poll rate is a phantom input stream that
//! drifts the cursor on its own.

/// The whole frame is kept, as long as the poll's read buffer. A precision
/// touchpad's click button, scan time and contact count come after five
/// finger slots, 20 and more bytes in; a 16-byte snapshot took a click under
/// a resting finger for a repeat of the rest and dropped it.
pub struct FrameRepeat {
    pub last_frame: [u8; FRAME_MAX],
    pub last_frame_len: usize,
    pub frame_repeats: u32,
}

/// The poll reads at most this much (input/poll/cycle.rs).
pub const FRAME_MAX: usize = 256;

impl Default for FrameRepeat {
    fn default() -> Self {
        Self { last_frame: [0; FRAME_MAX], last_frame_len: 0, frame_repeats: 0 }
    }
}
