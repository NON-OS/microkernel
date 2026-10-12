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

//! The responder's side of stage 2: it checks Ea itself before it
//! answers with Eb.

use super::peer::{Peer, A as A_, B as B_};
use nonos_bt_smp::funcs::{f5, f6};
use nonos_bt_smp::pdu::*;

impl Peer {
    pub(super) fn on_check(&mut self, ea: &[u8; 16]) -> Vec<Vec<u8>> {
        let (mac, ltk) = f5(&self.dh, &self.na, &self.nb, &A_, &B_);
        let mut r = [0u8; 16];
        if self.passkey_entry() {
            r[12..].copy_from_slice(&self.passkey.to_be_bytes());
        }
        let io_a = self.req.unwrap().f6_io_cap();
        if f6(&mac, &self.na, &self.nb, &r, &io_a, &A_, &B_) != *ea {
            return vec![vec![PAIRING_FAILED, DHKEY_CHECK_FAILED]];
        }
        self.ltk = ltk;
        let mut eb = f6(&mac, &self.nb, &self.na, &r, &self.ours.f6_io_cap(), &B_, &A_);
        eb[0] ^= self.faults.bad_check as u8;
        vec![frame(PAIRING_DHKEY_CHECK, &le(&eb)).unwrap()]
    }
}
