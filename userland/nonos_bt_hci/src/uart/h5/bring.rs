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

//! Bringing a Three-wire link up, and sending on it. Both waits are
//! bounded by a number of looks with a pause between them.

use alloc::collections::VecDeque;

use super::engine::H5;
use super::hci::H5Hci;
use super::header::PAYLOAD_MAX;
use crate::error::HciError;
use crate::uart::port::UartPort;
use nonos_bt_wire::WireError;

/// Looks, about a millisecond apart, a send waits for window room.
pub const SEND_LOOKS: u32 = 2000;

impl<P: UartPort> H5Hci<P> {
    pub fn port_mut(&mut self) -> &mut P {
        &mut self.port
    }

    /// `retx_ms`: how long a reliable packet waits to be acknowledged.
    pub fn new(port: P, retx_ms: u64) -> Self {
        let (events, acl) = (VecDeque::new(), VecDeque::new());
        Self { port, h5: H5::default(), events, acl, retx_ms, restarts: 0, dropped: 0 }
    }

    /// Run link establishment until Active, at most `looks` looks.
    pub fn establish(&mut self, looks: u32) -> Result<(), HciError> {
        for _ in 0..looks {
            self.pump()?;
            if self.h5.active() {
                self.restarts = self.h5.restarts;
                return Ok(());
            }
            self.port.pause();
        }
        Err(HciError::NoLink)
    }

    pub(super) fn send(&mut self, kind: u8, packet: &[u8]) -> Result<(), HciError> {
        if packet.len() > PAYLOAD_MAX {
            return Err(HciError::Wire(WireError::TooLong));
        }
        for _ in 0..SEND_LOOKS {
            if self.h5.send(kind, packet) {
                return self.pump();
            }
            self.pump()?;
            self.port.pause();
        }
        Err(HciError::NoLink)
    }
}
