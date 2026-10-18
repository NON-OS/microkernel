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

//! How often the last pointer report has come again unchanged. Polled
//! reads return the current report whether or not it is new; without this,
//! one stale motion report re-read at poll rate is a phantom input stream
//! that drifts the cursor on its own.

use crate::hid::TouchSample;
use crate::input::sample::MouseSample;

/// What the check keeps of the last report: the pointer sample decoded
/// from it, all the decision needs. Never the report's bytes, nor anything
/// of a report the driver does not read, a keyboard's above all (plan/50
/// C8). A touch sample holds the click button, so a click under a resting
/// finger is no repeat of the rest.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Seen {
    Touch(TouchSample),
    Mouse(MouseSample),
}

#[derive(Default)]
pub struct FrameRepeat {
    pub last: Option<Seen>,
    pub frame_repeats: u32,
}

impl FrameRepeat {
    /// A read that was no pointer report ends the run.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// The poll reads at most this much (input/poll/read_frame.rs).
pub const FRAME_MAX: usize = 256;
