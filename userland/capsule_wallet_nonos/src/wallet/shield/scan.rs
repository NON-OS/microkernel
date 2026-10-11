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

//! The restore scan (B7, v1.4.56), on the person's press only: one address
//! a request through the Shield's batches past the accounts, stopping after
//! GAP_LIMIT empty indices; "scan further" goes on up to MAX_HANDED. Pure.

use alloc::vec::Vec;

use super::scan_state::TRIES;
pub use super::scan_state::{Scan, ScanState, GAP_LIMIT, MAX_HANDED};

impl Scan {
    pub fn start(&mut self) {
        *self = Scan { state: ScanState::Deriving(0), ..Scan::default() };
    }

    /// The Shield's batch; an empty one ends the scan.
    pub fn batch(&mut self, batch: Vec<(u32, [u8; 20])>) {
        let more = !batch.is_empty();
        (self.batch, self.at, self.tries) = (batch, 0, 0);
        self.state = if more { ScanState::Reading } else { ScanState::Stopped };
    }

    pub fn next(&self) -> Option<[u8; 20]> {
        (self.state == ScanState::Reading).then(|| self.batch.get(self.at).map(|b| b.1)).flatten()
    }

    /// The read of the current address: its NOX, or None when it did not answer.
    pub fn read(&mut self, nox: Option<u128>) {
        let Some(&(index, address)) = self.batch.get(self.at) else { return };
        match nox {
            None if self.tries + 1 < TRIES => {
                self.tries += 1;
                return;
            }
            Some(n) if n > 0 => {
                self.found.push((index, address, n));
                self.empty_run = 0;
            }
            Some(_) => self.empty_run += 1,
            None => {}
        }
        (self.at, self.tries) = (self.at + 1, 0);
        let last = self.batch.last().map_or(0, |b| b.0);
        if self.empty_run >= GAP_LIMIT || last >= MAX_HANDED {
            self.state = ScanState::Stopped;
        } else if self.at == self.batch.len() {
            self.state = ScanState::Deriving(last + 1);
        }
    }

    /// "Scan further": the next batch past the last, if any is left.
    pub fn further(&mut self) {
        let last = self.batch.last().map_or(0, |b| b.0);
        if self.state == ScanState::Stopped && last < MAX_HANDED {
            (self.empty_run, self.state) = (0, ScanState::Deriving(last + 1));
        }
    }
}
