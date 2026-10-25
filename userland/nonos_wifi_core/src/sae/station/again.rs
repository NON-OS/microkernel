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

//! What a hostile or broken AP can make an exchange do again, bounded.
//! Each anti-clogging request is answered with the commit and its token,
//! at most `TOKENS_MAX` times. Once this station has confirmed, the AP's
//! own commit sent again means its confirm was lost: the station sends its
//! confirm again with the send-confirm counter one higher (IEEE Std
//! 802.11-2020, 12.4.8.6.5, recalled), at most `CONFIRMS_MAX` confirms in
//! all. Any other commit then is not the AP's and is ignored. Past either
//! bound the frame that crossed it is discarded and no work is done for it;
//! nothing unauthenticated ends the exchange, only the AP's confirm or the
//! driver's deadline.

use super::*;

/// Anti-clogging tokens taken in one exchange: an AP under load asks once. A
/// forger who keeps asking costs this many commits, not one per request
/// until the deadline.
pub const TOKENS_MAX: u8 = 3;
/// Confirms sent in one exchange, the first included.
pub const CONFIRMS_MAX: u16 = 3;

impl SaeStation {
    pub(super) fn commit_again(&mut self, body: &[u8]) -> SaeStep {
        let same = body
            .get(2..2 + 3 * LEN)
            .is_some_and(|b| b[..LEN] == self.peer_scalar[..] && b[LEN..] == self.peer_element[..]);
        if !same {
            return SaeStep::Ignore;
        }
        if self.confirms >= CONFIRMS_MAX {
            return self.discard(SaeFailure::Looping);
        }
        self.send_confirm()
    }

    /// This station's confirm, its send-confirm counter one past the last.
    pub(super) fn send_confirm(&mut self) -> SaeStep {
        self.confirms += 1;
        let c = confirm(
            &self.keys.kck,
            self.confirms,
            &self.own_scalar,
            &self.own_element,
            &self.peer_scalar,
            &self.peer_element,
        );
        let body = confirm_body(self.confirms, &c);
        SaeStep::Send { seq: SEQ_CONFIRM, status: STATUS_SUCCESS, body }
    }
}
