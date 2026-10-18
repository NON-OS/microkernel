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

//! A link that has gone quiet, and one whose PDUs find no buffer. A
//! controller that reset itself without a word, or dropped the link with no
//! Disconnection Complete, leaves a quiet link up for good: after
//! `QUIET_MS` of nothing, the controller is asked whether it still has the
//! link, and one that says it does not has lost its state. A PDU that
//! finds no buffer for longer than a link can go unanswered and stay up
//! (the 32 s greatest supervision timeout, Vol 6, Part B, 4.5.2) means
//! the controller is keeping buffers it has no use for, and the link ends.

use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};

const QUIET_MS: u64 = 10_000;
const CREDIT_MS: u64 = 32_000;
/// Unknown HCI Command (Vol 1, Part F, 2): no Read RSSI to ask with.
const UNKNOWN_COMMAND: u8 = 0x01;

impl Session {
    pub(super) fn alive<T: Transport>(
        &mut self,
        host: &mut Host<T>,
        now: u64,
    ) -> Result<(), HciError> {
        if core::mem::take(&mut self.short) {
            let since = *self.stuck_since.get_or_insert(now);
            if now.saturating_sub(since) >= CREDIT_MS {
                self.give_up("the controller gives no ACL buffers back");
            }
        }
        let since = *self.quiet_since.get_or_insert(now);
        if !self.probe || now.saturating_sub(since) < QUIET_MS {
            return Ok(());
        }
        self.quiet_since = Some(now);
        match host.link_known(self.handle) {
            Ok(true) => Ok(()),
            Ok(false) => Err(HciError::Reset),
            Err(HciError::Status(_, UNKNOWN_COMMAND)) => {
                self.probe = false;
                Ok(())
            }
            // Any other refusal says nothing of the link.
            Err(HciError::Status(..)) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
