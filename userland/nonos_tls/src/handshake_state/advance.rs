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

//! Taking the records that arrived since the last look.

use super::types::{HandshakeState, Progress};
use crate::handshake_step::{step, Step};
use crate::record_frame::{record_at, APPLICATION_DATA};

impl HandshakeState {
    /*
     * The flight is complete when a whole Finished has been decrypted, not
     * when some number of records has arrived: a server may put its entire
     * encrypted flight in one record, and counting to three waited for
     * records that were never coming. Each record is opened once however
     * often this is called, and a flight cut short anywhere, even on a record
     * boundary, reads as incomplete.
     */
    /// Decrypt what completed since the last call and say where the flight is.
    pub fn advance(&mut self, flight: &[u8]) -> Progress {
        if let Some(settled) = self.settled() {
            return settled;
        }
        while let Some((kind, end)) = record_at(flight, self.cursor) {
            let at = self.cursor;
            self.cursor = end;
            if kind == APPLICATION_DATA {
                if let Some(progress) = self.take(&flight[at..end], end) {
                    return progress;
                }
            } else if let Some(description) = crate::alert::description_in_record(&flight[at..]) {
                self.alert = Some(description);
                return Progress::Alert(description);
            }
        }
        Progress::Incomplete
    }

    fn take(&mut self, record: &[u8], end: usize) -> Option<Progress> {
        let k = &self.keys;
        let Some(plain) =
            crate::record_open::open(k.suite, &k.server_key, &k.server_iv, self.seq, record)
        else {
            self.broken = true;
            return Some(Progress::Broken);
        };
        self.seq += 1;
        match step(&plain) {
            Step::Messages(messages) => self.msgs.extend_from_slice(messages),
            Step::Stop(description) => {
                self.alert = Some(description);
                return Some(Progress::Alert(description));
            }
            Step::Ignore => {}
        }
        if self.finished_arrived() {
            self.end = Some(end);
            return Some(Progress::Complete(end));
        }
        None
    }
}
