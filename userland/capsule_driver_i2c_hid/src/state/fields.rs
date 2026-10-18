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

use alloc::vec::Vec;

use nonos_libc::Deadline;

use super::{FrameRepeat, LastRead};
use crate::drive::{Doorbell, Pace, Recover};
use crate::hid::{MouseLayout, Multitouch, TouchLayout};
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
    /// The buffer each frame is read into, as long as the poll's read. It is
    /// cleared once the frame's report is handed on (plan/50 C8).
    pub frame: Vec<u8>,
    /// What the last read of the input register brought.
    pub last_read: LastRead,
    /// The last pointer sample and how often it has come again unchanged.
    pub repeat: FrameRepeat,
    // The absolute field map from the report descriptor, and the gesture state
    // that turns its reports into pointer events. Empty for a relative mouse.
    pub touch_layout: TouchLayout,
    /// Every finger: a touchscreen, or a pad that reports them in parallel.
    pub mt: Multitouch,
    /// The relative mouse collection, parsed from the same descriptor: what a
    /// pad reports while it stays in mouse mode (it refused input mode 3, or
    /// has no precision touchpad collection).
    pub mouse_layout: MouseLayout,
    pub gesture: TouchGesture,
    /// The pad's interrupt line, where the controller senses one.
    pub doorbell: Doorbell,
    /// When the loop next reads the pad, slower while it is quiet.
    pub pace: Pace,
    /// Whether touchpad mode was set, and setting it again after a reset.
    pub recover: Recover,
    /// Nothing is left to drive: no pad came up in the window given, or the
    /// device is no touchpad or mouse. The server loop leaves.
    pub done: bool,
    /// When the search for a pad ends (setup/run.rs).
    pub probe_window: Deadline,
    /// The line that setup is retrying was already printed.
    pub said_retrying: bool,
}
