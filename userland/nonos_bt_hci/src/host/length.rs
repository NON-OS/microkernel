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

//! LE Data Length Extension on the host's side. At bring-up, a controller
//! with the feature is asked for its maximum; on each new connection the
//! host asks for that much, at most 251 octets in 2120 µs, the time 251
//! octets take on the 1M PHY. A controller that refuses either command, or
//! answers outside the specification's range, leaves the link at the
//! default 27 octets: nothing fails for it.

use super::masks::DATA_LENGTH_EXTENSION;
use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::data_length::{set_data_length, MaxDataLength};
use nonos_bt_wire::hci::data_length::{LE_READ_MAXIMUM_DATA_LENGTH, LE_SET_DATA_LENGTH};

/// The most time a connection is asked for. The octets need no bound of
/// their own: a maximum in range is at most 251 already.
pub const ASK_TIME_US: u16 = 2120;

impl<T: Transport> Host<T> {
    /// What to ask each connection for, given the controller's LE features:
    /// `None` without the feature, or when the controller refuses or answers
    /// out of range.
    pub(super) fn data_length_to_ask(
        &mut self,
        le_features: u64,
    ) -> Result<Option<(u16, u16)>, HciError> {
        if le_features & DATA_LENGTH_EXTENSION == 0 {
            return Ok(None);
        }
        let ret = match self.command(LE_READ_MAXIMUM_DATA_LENGTH, &[]) {
            Ok(ret) => ret,
            Err(HciError::Status(..)) => return Ok(None),
            Err(e) => return Err(e),
        };
        let Ok(m) = MaxDataLength::parse(&ret) else { return Ok(None) };
        Ok(Some((m.tx_octets, m.tx_time.min(ASK_TIME_US))))
    }

    /// Ask connection `handle` for the longer PDUs, if the controller can
    /// carry them. A refusal changes nothing.
    pub(super) fn ask_data_length(&mut self, handle: u16) -> Result<(), HciError> {
        let Some((octets, time)) = self.data_length else { return Ok(()) };
        let params = set_data_length(handle, octets, time)?;
        match self.command(LE_SET_DATA_LENGTH, &params) {
            Ok(_) | Err(HciError::Status(..)) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
