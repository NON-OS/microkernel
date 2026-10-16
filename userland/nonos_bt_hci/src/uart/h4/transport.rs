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

//! The host core's view of an H4 port.

use alloc::vec::Vec;

use super::hci::H4Hci;
use super::kind::{ACL, COMMAND};
use crate::error::HciError;
use crate::transport::Transport;
use crate::uart::port::UartPort;

impl<P: UartPort> H4Hci<P> {
    pub fn port_mut(&mut self) -> &mut P {
        &mut self.port
    }

    pub fn set_acl_max(&mut self, acl_max: u16) {
        self.reader.set_acl_max(acl_max);
    }

    /// Packets dropped because the host left too many unread.
    pub fn dropped(&self) -> u32 {
        self.dropped
    }

    fn send(&mut self, kind: u8, packet: &[u8]) -> Result<(), HciError> {
        let framed = [&[kind][..], packet].concat();
        self.port.write(&framed).map_err(HciError::Transport)
    }
}

impl<P: UartPort> Transport for H4Hci<P> {
    fn send_command(&mut self, packet: &[u8]) -> Result<(), HciError> {
        self.send(COMMAND, packet)
    }
    fn send_acl(&mut self, packet: &[u8]) -> Result<(), HciError> {
        self.send(ACL, packet)
    }
    fn poll_event(&mut self) -> Result<Option<Vec<u8>>, HciError> {
        self.pump()?;
        Ok(self.events.pop_front())
    }
    fn poll_acl(&mut self) -> Result<Option<Vec<u8>>, HciError> {
        self.pump()?;
        Ok(self.acl.pop_front())
    }
    fn pause(&mut self) {
        self.port.pause()
    }
}
