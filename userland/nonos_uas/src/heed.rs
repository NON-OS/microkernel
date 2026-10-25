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
//! What an IU on the status pipe does to the command in flight. One that
//! names another tag is no command in flight (one before it, ended or
//! given up) and is let go and counted, as is any after this one ended, so
//! neither is taken for this command's status; the status pipe is read
//! again, up to DROPS_MAX times, so a device that sends nothing else is
//! not waited on for ever.

use crate::command::{DataDir, ScsiCommand};
use crate::exchange::Exchange;
use crate::status::StatusIu;
use crate::step::Step;

const OUT_OF_TURN: u8 = 0x02;
pub const DROPS_MAX: u8 = 8;

impl Exchange {
    /// The step after `iu` came on the status pipe.
    pub fn status(&mut self, iu: StatusIu, cmd: &mut dyn ScsiCommand) -> Step {
        if self.ended || iu.tag() != self.tag {
            return self.let_go();
        }
        // With streams the data has moved before the status pipe is read,
        // so a READY then is out of turn by `moved` alone.
        let ready = |want: DataDir| !self.moved && self.dir == want;
        match iu {
            StatusIu::ReadReady { .. } if ready(DataDir::In) => Step::DataIn,
            StatusIu::WriteReady { .. } if ready(DataDir::Out) => Step::DataOut,
            StatusIu::Sense { status, sense, .. } => {
                cmd.finished(status, sense);
                self.ended = true;
                Step::Done
            }
            StatusIu::Response { code, .. } => Step::Failed(code),
            _ => Step::Failed(OUT_OF_TURN),
        }
    }

    fn let_go(&mut self) -> Step {
        self.dropped = self.dropped.saturating_add(1);
        match (self.ended, self.dropped > DROPS_MAX) {
            (true, _) => Step::Done,
            (false, true) => Step::Failed(OUT_OF_TURN),
            (false, false) => Step::Status,
        }
    }
}
