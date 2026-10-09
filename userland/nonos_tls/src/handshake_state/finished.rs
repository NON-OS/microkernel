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

//! Whether a whole Finished is among the messages decrypted so far.

use super::types::{HandshakeState, Progress};
use crate::scan_messages::FINISHED;

impl HandshakeState {
    /// Walk the messages not yet looked at. True once a Finished with all of
    /// its declared length is present.
    pub(super) fn finished_arrived(&mut self) -> bool {
        while let Some(head) = self.msgs.get(self.scanned..self.scanned + 4) {
            let len = ((head[1] as usize) << 16) | ((head[2] as usize) << 8) | head[3] as usize;
            let end = self.scanned + 4 + len;
            if end > self.msgs.len() {
                return false;
            }
            if head[0] == FINISHED {
                return true;
            }
            self.scanned = end;
        }
        false
    }

    /// The answer already reached, which later bytes cannot change.
    pub(super) fn settled(&self) -> Option<Progress> {
        if self.broken {
            return Some(Progress::Broken);
        }
        if let Some(description) = self.alert {
            return Some(Progress::Alert(description));
        }
        self.end.map(Progress::Complete)
    }

    /// The server's alert, if it stopped with one.
    pub fn alert(&self) -> Option<u8> {
        self.alert
    }
}
