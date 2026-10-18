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
//! Encrypting a link as central (Vol 4, Part E, 7.8.24) with the LTK of
//! LE Secure Connections pairing, whose EDIV and Rand are zero (Vol 3,
//! Part H, 2.4.4). The answer is Encryption Change (Vol 4, Part E, 7.7.8).

use alloc::vec::Vec;

use super::ltk::enable_encryption_params;
use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::encryption::ENCRYPTION_ON_AES_CCM;
use nonos_bt_wire::hci::event::parse_event;
use nonos_bt_wire::hci::event_code::{DISCONNECTION_COMPLETE, ENCRYPTION_CHANGE};
use nonos_bt_wire::hci::{opcode as op, DisconnectionComplete, EncryptionChange};

enum End {
    Change(EncryptionChange),
    /// The link went down first; the event, kept for the caller, and why.
    Dropped(Vec<u8>, u8),
}

impl<T: Transport> Host<T> {
    /// `Ok` once the controller reports the link encrypted with AES-CCM.
    /// A refusal is `Status` with the controller's code, a link that drops
    /// meanwhile `Status` with the disconnection reason (its Disconnection
    /// Complete stays queued), and an Encryption Change with status 0 that
    /// leaves the link unencrypted is `Status` 0.
    pub fn encrypt(&mut self, handle: u16, ltk: &[u8; 16]) -> Result<(), HciError> {
        let start = op::LE_ENABLE_ENCRYPTION;
        self.command_pending(start, &enable_encryption_params(handle, ltk)?)?;
        match self.wait_for(|e| end(e, handle))? {
            Some(End::Change(c)) if c.status == 0 && c.enabled == ENCRYPTION_ON_AES_CCM => Ok(()),
            Some(End::Change(c)) => Err(HciError::Status(start, c.status)),
            Some(End::Dropped(event, reason)) => {
                self.keep(event)?;
                Err(HciError::Status(start, reason))
            }
            None => Err(HciError::Timeout(start)),
        }
    }
}

fn end(event: &[u8], handle: u16) -> Result<Option<End>, HciError> {
    let ev = parse_event(event)?;
    Ok(match ev.code {
        ENCRYPTION_CHANGE => Some(EncryptionChange::parse(ev.params)?)
            .filter(|c| c.handle == handle)
            .map(End::Change),
        DISCONNECTION_COMPLETE => Some(DisconnectionComplete::parse(ev.params)?)
            .filter(|d| d.status == 0 && d.handle == handle)
            .map(|d| End::Dropped(event.to_vec(), d.reason)),
        _ => None,
    })
}
