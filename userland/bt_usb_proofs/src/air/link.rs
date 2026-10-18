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

//! A new link as the peer meets it, and the SMP PDUs it is sent.

use super::state::{Air, OURS};
use nonos_bt_smp::pdu::{IDENTITY_ADDRESS_INFORMATION, IDENTITY_INFORMATION, PAIRING_REQUEST};
use nonos_bt_wire::l2cap::cid::SMP;

/// `addr` in f5's form (Vol 3, Part H, 2.2.7): type, then most significant
/// octet first.
fn f5_form(kind: u8, addr: &[u8; 6]) -> [u8; 7] {
    let mut out = [kind; 7];
    for (o, a) in out[1..].iter_mut().zip(addr.iter().rev()) {
        *o = *a;
    }
    out
}

impl Air {
    /// The host connects as `own` (Vol 4, Part E, 7.8.12): its public
    /// address, or the random one it set. The peer's pairing starts over;
    /// its keys, kept from an earlier pairing, stay.
    pub(super) fn new_link(&mut self, own: u8) {
        let host = match (own, self.seen.random) {
            (0, _) => OURS,
            (1, Some(r)) => r,
            _ => panic!("connected as address type {own} with no such address"),
        };
        self.seen.own_types.push(own);
        self.smp.ab = (f5_form(own, &host), f5_form(1, &self.seen.from));
        (self.smp.req, self.smp.round) = (None, 0);
        (self.ltk, self.seen.paired_here) = (None, false);
    }

    pub(super) fn smp_from_host(&mut self, pdu: &[u8]) {
        self.seen.paired_here = true;
        match pdu.first() {
            Some(&PAIRING_REQUEST) => self.seen.requests += 1,
            Some(&IDENTITY_INFORMATION | &IDENTITY_ADDRESS_INFORMATION) => {
                self.seen.identity.push(pdu.to_vec());
            }
            _ => {}
        }
        if let Some(k) = self.shown.get() {
            self.smp.passkey = k;
        }
        for reply in self.smp.on(pdu) {
            self.send_to_host(SMP, &reply);
        }
    }
}
