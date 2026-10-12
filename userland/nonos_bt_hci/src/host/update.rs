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

//! LE Connection Update (Vol 4, Part E, 7.8.18): new timing for a link,
//! which a central sends once it has accepted a peripheral's Connection
//! Parameter Update Request (Vol 3, Part A, 4.20). Accepting a request
//! and not sending this would leave the peripheral waiting on timing that
//! never comes. The outcome, LE Connection Update Complete, is an event
//! the caller meets later; nothing waits for it here.

use alloc::vec::Vec;

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::{opcode as op, HANDLE_MAX};
use nonos_bt_wire::l2cap::ConnParams;
use nonos_bt_wire::WireError;

/// The 14 parameter octets: handle, the four timing values, and the
/// connection event length hint left to the controller (0, 0), or
/// `Reserved` for a handle or timing outside the allowed ranges.
pub fn connection_update_params(handle: u16, p: &ConnParams) -> Result<Vec<u8>, HciError> {
    if handle > HANDLE_MAX || !p.acceptable() {
        return Err(HciError::Wire(WireError::Reserved));
    }
    let mut out = Vec::with_capacity(14);
    out.extend_from_slice(&handle.to_le_bytes());
    for v in [p.interval_min, p.interval_max, p.latency, p.timeout] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(&[0; 4]);
    Ok(out)
}

impl<T: Transport> Host<T> {
    /// `Ok` once the controller has taken the update (Command Status).
    pub fn update_link(&mut self, handle: u16, p: &ConnParams) -> Result<(), HciError> {
        let params = connection_update_params(handle, p)?;
        self.command_pending(op::LE_CONNECTION_UPDATE, &params)
    }
}
