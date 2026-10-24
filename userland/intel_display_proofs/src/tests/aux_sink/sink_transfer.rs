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

//! One transfer as the sink takes it. An AUX fault first: a transfer not
//! answered cleanly changes no I2C state. Then the I2C request
//! (`sink_i2c.rs`). A request without middle-of-transaction is the stop;
//! the next request with it opens a new transaction.

use super::sink::{AuxSink, Try};

impl AuxSink {
    /// The reply, command byte first; None when no reply comes.
    pub fn transfer(&mut self, req: &[u8], now: u64) -> Option<Vec<u8>> {
        self.transfers += 1;
        let mot = req[0] & 0x40 != 0;
        if !self.open || (self.stopping && mot) {
            self.close(now);
            let fault = self.faults.get(self.log.len()).copied().unwrap_or_default();
            self.log.push(Try { fault, t0: now, ..Default::default() });
            (self.open, self.stopping, self.segment, self.pos) = (true, false, 0, 0);
        }
        self.stopping |= !mot;
        if let Some(r) = self.aux_fault(req) {
            return r;
        }
        let r = self.i2c(req);
        let t = self.log.last_mut().unwrap();
        if !mot && r.first() == Some(&0) {
            t.stop_ok = true;
        }
        Some(r)
    }

    /// End the transaction in hand, if one is.
    pub fn close(&mut self, now: u64) {
        if let (true, Some(t)) = (self.open, self.log.last_mut()) {
            t.t1 = now;
        }
        self.open = false;
    }
}
