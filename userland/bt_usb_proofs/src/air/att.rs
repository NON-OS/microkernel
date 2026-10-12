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

//! ATT from the host. Its answers to the peer's own requests (odd
//! opcodes, Vol 3, Part F, 3.4.8) are kept for the proofs; its requests
//! go to the peripheral, and only once the link is encrypted, since a HID
//! service is never read on a clear link (HOGP 1.0, 4.1).

use super::state::Air;
use nonos_bt_wire::l2cap::cid::ATT;

impl Air {
    pub(super) fn att_from_host(&mut self, pdu: &[u8]) {
        if pdu[0] & 1 == 1 {
            self.att_answers.push(pdu.to_vec());
            return;
        }
        assert!(self.ltk.is_some(), "the host asked over ATT on a clear link");
        if let Some(answer) = self.hid.on(pdu) {
            self.send_to_host(ATT, &answer);
        }
    }
}
