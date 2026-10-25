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

//! What the shell last told the compositor of the kept displays
//! (state/display_keep.rs), so a kept value is sent once per change in the
//! store and once per compositor.

use super::display_keep::valid;

/// What the compositor was last told of each field, and which compositor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Replay {
    told: [Option<u8>; 5],
    compositor: u32,
}

impl Replay {
    /// The compositor running now is `pid`; a new one was told nothing.
    pub fn compositor(&mut self, pid: u32) {
        if pid != self.compositor {
            *self = Replay { told: [None; 5], compositor: pid };
        }
    }

    /// The value to send for field `i` read as `read`, if any.
    pub fn due(&self, i: usize, read: Option<u8>) -> Option<u8> {
        let v = read.filter(|&v| valid(i, v))?;
        (self.told.get(i)? != &Some(v)).then_some(v)
    }

    /// The compositor answered for field `i` set to `v`, taken or refused:
    /// either way it is not asked again until the store changes.
    pub fn told(&mut self, i: usize, v: u8) {
        if let Some(t) = self.told.get_mut(i) {
            *t = Some(v);
        }
    }
}
