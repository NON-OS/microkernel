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

//! A deauthentication or disassociation from the access point. Without
//! management frame protection it ends the link as it comes. With it, only a
//! protected one does: under the pairwise key when sent to this station, under
//! BIP when sent to all. One in the clear could be anyone's. An access point
//! that rebooted and forgot the association also sends one in the clear, with
//! reason 6 or 7, so that one starts an SA Query, as wpa_supplicant does; the
//! link ends only if the query goes unanswered. Any other is dropped.

use crate::dot11::header::{FC_PROTECTED, MAC_HEADER_LEN};
use crate::station::{LinkStation, Loss, Rx, RxDrop};

/// A class 2 frame from a station not authenticated.
const REASON_NOT_AUTHENTICATED: u16 = 6;
/// A class 3 frame from a station not associated.
const REASON_NOT_ASSOCIATED: u16 = 7;

impl LinkStation {
    pub(super) fn on_leave(&mut self, mpdu: &[u8], hw: bool, group: bool) -> Rx {
        let protected = u16::from_le_bytes([mpdu[0], mpdu[1]]) & FC_PROTECTED != 0;
        if !self.pmf() {
            return match reason(mpdu, MAC_HEADER_LEN) {
                Some(r) if !protected => Rx::Lost(self.lose(Loss::Left(r))),
                _ => Rx::Dropped(RxDrop::Malformed),
            };
        }
        let taken = if group {
            self.bip(mpdu).then(|| reason(mpdu, MAC_HEADER_LEN)).flatten()
        } else if protected {
            match self.open_mgmt(mpdu, hw) {
                Ok(body) => reason(&body, 0),
                Err(why) => return Rx::Dropped(why),
            }
        } else {
            None
        };
        if let Some(r) = taken {
            return Rx::Lost(self.lose(Loss::Left(r)));
        }
        match reason(mpdu, MAC_HEADER_LEN) {
            Some(r @ (REASON_NOT_AUTHENTICATED | REASON_NOT_ASSOCIATED)) if !protected => {
                self.ask(r)
            }
            _ => Rx::Dropped(RxDrop::Unprotected),
        }
    }
}

/// The reason code at `at`, the first field of the body.
fn reason(b: &[u8], at: usize) -> Option<u16> {
    let r = b.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes([r[0], r[1]]))
}
