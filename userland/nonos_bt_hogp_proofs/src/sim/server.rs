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

//! A peripheral's ATT server over a `Db`, written from Vol 3, Part F,
//! 3.4, with the default LE ATT_MTU of 23 (Vol 3, Part F, 3.2.8). It keeps
//! every write, and can be told to misbehave.

use super::db::Db;

pub const MTU: usize = 23;

pub struct Peripheral {
    pub db: Db,
    /// (handle, value, as a Write Command).
    pub writes: Vec<(u16, Vec<u8>, bool)>,
    /// Answer every request with this error.
    pub refuse: Option<u8>,
    /// Answer nothing.
    pub silent: bool,
    /// Answer requests with this opcode with this error, the rest as usual.
    pub refuse_op: Option<(u8, u8)>,
    /// Answer a Read of this handle with this error.
    pub refuse_read: Option<(u16, u8)>,
    /// Answer nothing to requests with this opcode.
    pub mute: Option<u8>,
    /// Answer every Find By Type Value with the groups from handle 1, so
    /// the second answer goes back over the first.
    pub repeat: bool,
    /// Answer Find Information past the end of the range asked for.
    pub wide: bool,
    pub requests: usize,
}

impl Peripheral {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            writes: Vec::new(),
            refuse: None,
            silent: false,
            refuse_op: None,
            refuse_read: None,
            mute: None,
            repeat: false,
            wide: false,
            requests: 0,
        }
    }
}
