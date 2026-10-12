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

//! The knobs of the simulated controller.

pub const ADDR: [u8; 6] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

pub struct Config {
    pub le: bool,
    /// LE Read Buffer Size's length and count.
    pub le_buffers: (u16, u8),
    /// Read Buffer Size's ACL length and count.
    pub buffers: (u16, u16),
    /// Answer nothing at all.
    pub silent: bool,
    /// Answer this opcode with this status instead.
    pub fail: Option<(u16, u8)>,
    /// Events sent ahead of each answer.
    pub before: Vec<Vec<u8>>,
    /// For these opcodes, these events instead of the usual answer: the
    /// Command Status and whatever the command brings after it.
    pub script: Vec<(u16, Vec<Vec<u8>>)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            le: true,
            le_buffers: (27, 4),
            buffers: (0, 0),
            silent: false,
            fail: None,
            before: Vec::new(),
            script: Vec::new(),
        }
    }
}

impl Config {
    /// The default controller with these opcodes answered by script.
    pub fn scripted(script: Vec<(u16, Vec<Vec<u8>>)>) -> Self {
        Self { script, ..Self::default() }
    }
}
