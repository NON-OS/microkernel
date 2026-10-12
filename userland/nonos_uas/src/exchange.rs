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
//! device says READ READY or WRITE READY on the status pipe before the
//! data moves; at SuperSpeed the data moves on the command's stream at
//! once. Either way a Sense IU with its tag ends it, and a Response IU, or
//! any IU not for it, fails it.

use crate::command::{DataDir, ScsiCommand};
use crate::status::StatusIu;
use crate::step::Step;

const OUT_OF_TURN: u8 = 0x02;

pub struct Exchange {
    tag: u16,
    dir: DataDir,
    streams: bool,
    moved: bool,
}

impl Exchange {
    pub fn new(tag: u16, dir: DataDir, streams: bool) -> Exchange {
        Exchange { tag, dir, streams, moved: false }
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

    /// The step after `iu` came on the status pipe.
    pub fn status(&mut self, iu: StatusIu, cmd: &mut dyn ScsiCommand) -> Step {
        let ready = |want: DataDir| !self.streams && !self.moved && self.dir == want;
        match iu {
            StatusIu::ReadReady { tag } if tag == self.tag && ready(DataDir::In) => Step::DataIn,
            StatusIu::WriteReady { tag } if tag == self.tag && ready(DataDir::Out) => Step::DataOut,
            StatusIu::Sense { tag, status, sense } if tag == self.tag => {
                cmd.finished(status, sense);
                Step::Done
            }
            StatusIu::Response { tag, code } if tag == self.tag => Step::Failed(code),
            _ => Step::Failed(OUT_OF_TURN),
        }
    }
}
