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

//! Feed one received frame to the state machine and return what to send
//! next, counting what the join saw on the way.

use alloc::vec::Vec;

use super::classify::{classify, wrap_eapol, RxKind};
use super::counters::Counters;
use crate::dot11::auth::{SUBTYPE_DEAUTH, SUBTYPE_DISASSOC};
use crate::dot11::header::fc_subtype;
use crate::mlme::Mlme;

pub(super) fn feed(m: &mut Mlme, rx: &[u8], c: &mut Counters) -> Option<Vec<u8>> {
    let fc = u16::from_le_bytes([*rx.first()?, *rx.get(1)?]);
    match classify(rx) {
        RxKind::Mgmt => {
            let sub = fc_subtype(fc);
            if sub == SUBTYPE_DEAUTH || sub == SUBTYPE_DISASSOC {
                c.deauth += 1;
            }
            m.on_mgmt(rx).tx
        }
        RxKind::Eapol(payload) => {
            count_data(rx, m, c);
            c.eapol += 1;
            let reply = m.on_eapol(payload).tx?;
            wrap_eapol(&reply, m.our_mac(), m.bssid())
        }
        RxKind::Other if rx.len() >= 10 && fc & 0x000C == 0x0008 => {
            count_data(rx, m, c);
            if c.probe == 0 {
                c.probe = u32::from(fc) | (rx.len().min(0xFFFF) as u32) << 16;
            }
            None
        }
        RxKind::Other => None,
    }
}

fn count_data(rx: &[u8], m: &Mlme, c: &mut Counters) {
    c.data += 1;
    if rx[4..10] == m.our_mac() {
        c.to_us += 1;
    }
}
