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

//! The gesture engine's state between frames, and what one frame asks the
//! driver to post.

#[derive(Default)]
pub struct TouchGesture {
    pub(super) was_tip: bool,
    /// The clickpad's button is down, as the last debounced edge said.
    pub(crate) was_button: bool,
    /// Consecutive samples disagreeing with `was_button`, for debounce.
    pub(super) button_run: u8,
    pub(super) scrolling: bool,
    pub(super) scroll_y: u32,
    /// The finger the scroll follows, when the pad names its contacts.
    pub(super) scroll_id: Option<u32>,
    /// The contact count of the current frame. In hybrid reporting only the
    /// first report of a frame carries it and the others say zero.
    pub(super) frame_contacts: u32,
    pub(super) multi_touch: bool,
    pub(super) palm: bool,
    pub(super) last_x: i64,
    pub(super) last_y: i64,
    // Sub-pixel numerators carried between reports so slow motion below one
    // output pixel per report is accumulated instead of truncated away.
    pub(super) acc_x: i64,
    pub(super) acc_y: i64,
    // Tap-to-click tracking for the current single-finger touch: how many
    // reports the finger has been down, how far it has travelled, and whether
    // the touch is still eligible to become a tap (a physical press or a
    // palm/multi-touch cancels it).
    pub(super) tip_frames: u16,
    pub(super) tap_travel: i64,
    pub(super) tap_ok: bool,
}

/// What one sample should do. The driver posts these as input events.
#[derive(Default, PartialEq, Eq, Debug)]
pub struct TouchActions {
    /// Relative cursor motion in nominal screen pixels, already accelerated.
    pub motion: Option<(i32, i32)>,
    /// Wheel notches, positive to scroll up as a mouse wheel turned away
    /// from the user does.
    pub wheel: i32,
    pub button_down: bool,
    pub button_up: bool,
}
