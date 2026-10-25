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

//! A management frame that came in through `receive`: checked and acted on
//! as `receive_mgmt` does it, and answered in `Rx`'s terms. An SA Query a
//! deauthentication in the clear calls for is asked here at once, under the
//! station's own identifiers, and a frame of the AP's that the link does not
//! act on, or a robust one to all that does not hold under BIP, is said so.

use super::mgmt_rx::{leave_reason, MgmtDrop, MgmtRx};
use super::{LinkStation, Loss, Rx, RxDrop};
use crate::dot11::header::{MAC_HEADER_LEN, SUBTYPE_BEACON};
use crate::pmf::SUBTYPE_ACTION;

impl LinkStation {
    pub(super) fn on_mgmt(&mut self, f: &[u8], hw: bool) -> Rx {
        let sub = f.first().map_or(0, |b| b >> 4);
        if sub == SUBTYPE_BEACON && self.pmf.bigtk.is_some() {
            // Under beacon protection only a verified beacon is the AP heard.
            if self.receive_beacon(f) == super::BeaconRx::Verified {
                self.watch.heard();
            }
            return Rx::Dropped(RxDrop::Management);
        }
        let group = f.get(4).is_some_and(|a| a & 0x01 != 0);
        match self.receive_mgmt(f, hw) {
            MgmtRx::Left(reason) => Rx::Lost(Loss::Left(reason)),
            MgmtRx::Reply(frame) => Rx::Send(frame),
            MgmtRx::SaQueryNeeded => {
                let reason = f.get(MAC_HEADER_LEN..).and_then(|b| leave_reason(sub, b));
                self.ask(reason.unwrap_or(0))
            }
            MgmtRx::Confirmed => {
                self.watch.asked = None;
                Rx::Dropped(RxDrop::Management)
            }
            MgmtRx::Dropped(why) => Rx::Dropped(rx_drop(why, sub, group)),
        }
    }
}

fn rx_drop(why: MgmtDrop, sub: u8, group: bool) -> RxDrop {
    match why {
        MgmtDrop::NotAssociated => RxDrop::NotAssociated,
        MgmtDrop::Malformed => RxDrop::Malformed,
        MgmtDrop::NotOurs | MgmtDrop::NotHandled => RxDrop::Management,
        // A robust frame to all is only ever under BIP; one that does not
        // hold there is as good as sent in the clear.
        MgmtDrop::Undecryptable | MgmtDrop::Replay if group => RxDrop::Unprotected,
        MgmtDrop::Undecryptable => RxDrop::Undecryptable,
        MgmtDrop::Replay => RxDrop::Replay,
        // An action frame in the clear is one the link does not act on.
        MgmtDrop::Unprotected if sub == SUBTYPE_ACTION => RxDrop::Management,
        MgmtDrop::Unprotected => RxDrop::Unprotected,
    }
}
