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
//! One command's exchange, step by step. The Command IU sent, a USB 2
//! One command's exchange, step by step. The Command IU sent, a USB 2
//! device says READ READY or WRITE READY on the status pipe before the
//! data moves; at SuperSpeed the data moves on the command's stream at
//! once. Either way a Sense IU with its tag ends it and a Response IU with
//! its tag fails it (heed.rs); an IU for another tag, or one after the
//! command ended, is let go and counted.

use crate::command::DataDir;
use crate::step::Step;

pub struct Exchange {
    pub(crate) tag: u16,
    pub(crate) dir: DataDir,
    pub(crate) streams: bool,
    pub(crate) moved: bool,
    pub(crate) ended: bool,
    pub(crate) dropped: u8,
}

impl Exchange {
    pub fn new(tag: u16, dir: DataDir, streams: bool) -> Exchange {
        Exchange { tag, dir, streams, moved: false, ended: false, dropped: 0 }
    }

    /// The step after the Command IU went out.
    pub fn sent(&self) -> Step {
        match (self.streams, self.dir) {
            (true, DataDir::In) => Step::DataIn,
            (true, DataDir::Out) => Step::DataOut,
            _ => Step::Status,
        }
    }

    /// The step after the data moved.
    pub fn moved(&mut self) -> Step {
        self.moved = true;
        Step::Status
    }

    /// The IUs let go: for no command in flight, or after this one ended.
    pub fn dropped(&self) -> u8 {
        self.dropped
    }
}
