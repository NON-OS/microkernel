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

//! STATE (0x12): 32 bytes of u32: generation, width, height, serial, hint
//! width, hint height, flags, reserved. A driver reading device events
//! does so on this op; a lost output is re-programmed onto the current
//! primary before the reply, and the compositor repaints all of it.

use crate::bytes::{put, u32_at};
use crate::errno::{Errno, EINVAL};

pub const STATE_LEN: usize = 32;

pub const SCANNING_OUT: u32 = 1 << 0;
pub const LOST: u32 = 1 << 1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Bumped on every change the compositor must look at.
    pub generation: u32,
    pub width: u32,
    pub height: u32,
    pub serial: u32,
    /// The host's wanted size, 0 x 0 when it gives none.
    pub hint_width: u32,
    pub hint_height: u32,
    pub flags: u32,
}

impl State {
    pub fn encode(&self) -> [u8; STATE_LEN] {
        let mut b = [0u8; STATE_LEN];
        let f = [
            self.generation,
            self.width,
            self.height,
            self.serial,
            self.hint_width,
            self.hint_height,
            self.flags,
        ];
        for (i, v) in f.iter().enumerate() {
            put(&mut b, i * 4, &v.to_le_bytes());
        }
        b
    }

    pub fn decode(b: &[u8]) -> Result<Self, Errno> {
        let f = |i: usize| u32_at(b, i * 4).ok_or(EINVAL);
        Ok(State {
            generation: f(0)?,
            width: f(1)?,
            height: f(2)?,
            serial: f(3)?,
            hint_width: f(4)?,
            hint_height: f(5)?,
            flags: f(6)?,
        })
    }
}
