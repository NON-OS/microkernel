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

use super::FrameRepeat;
use crate::hid::{MouseLayout, TouchLayout};
use crate::input::TouchGesture;

pub struct State {
    pub i2c_port: u32,
    pub i2c_pid: u32,
    pub addr: u8,
    pub descriptor: [u8; 30],
    pub descriptor_len: usize,
    pub input_register: u16,
    pub input_len: usize,
    pub last_buttons: u8,
    pub probes: u64,
    pub input_polls: u64,
    pub input_reports: u64,
    pub post_failures: u64,
    pub woke: bool,
    /// The touch field map has decoded at least one real report. Until then
    /// the relative fallback stays available: a device that never streams
    /// its absolute collection must not go silent. Once proven, unmatched
    /// frames are dropped instead of being misread as boot-mouse packets.
    pub touch_decoded: bool,
    /// Polls since the last successful touch decode. When the absolute
    /// stream goes quiet for long enough (mode changed back, device reset),
    /// the relative fallback reopens rather than staying muted forever.
    pub polls_since_touch: u32,
    /// The GPIO doorbell has fired at least once, proving the platform's
    /// interrupt-status bit really tracks this pad. From then on the driver
    /// reads the i2c input register only on a fired doorbell: exact
    /// interrupt pacing, no stale re-reads. Until proven, timed polling
    /// continues so a doorbell that never latches cannot silence input.
    pub doorbell_proven: bool,
    /// The last raw frame and how often it has repeated verbatim.
    pub repeat: FrameRepeat,
    /// Raw frames already dumped to the boot console (bounded one-shot).
    pub frame_dumps: u32,
    // The absolute field map from the report descriptor, and the gesture state
    // that turns its reports into pointer events. Empty for a relative mouse.
    pub touch_layout: TouchLayout,
    /// The relative mouse collection, parsed from the same descriptor: what a
    /// pad reports while it stays in mouse mode (it refused input mode 3, or
    /// has no precision touchpad collection).
    pub mouse_layout: MouseLayout,
    pub gesture: TouchGesture,
    /// The controller has said it has no doorbell (or failed to answer it
    /// several times running): reads go by timer and it is not asked again.
    pub doorbell_absent: bool,
    pub doorbell_failures: u32,
    /// Reads made because the doorbell said a report waited that found
    /// none, in a row. A line that reads asserted with nothing pending is
    /// the wrong pin or the wrong polarity, and trusting it would hold reads
    /// back exactly while the finger moves.
    pub doorbell_misses: u32,
    /// The last input read returned a report (a non-zero length).
    pub last_read_had_report: bool,
    /// A plain-language console line about this pad was already printed.
    pub said: bool,
}
