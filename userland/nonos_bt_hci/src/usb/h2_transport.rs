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

//! `UsbHci` as the host core's transport.

use super::h2::{take, Polled, UsbHci, E_IO};
use super::pipes::{UsbPipes, COMMAND};
use crate::error::HciError;
use crate::transport::Transport;

impl<P: UsbPipes> Transport for UsbHci<P> {
    fn send_command(&mut self, packet: &[u8]) -> Result<(), HciError> {
        self.pipes.control_out(COMMAND, packet).map_err(HciError::Transport)
    }
    fn send_acl(&mut self, packet: &[u8]) -> Result<(), HciError> {
        match self.pipes.bulk_out(packet).map_err(HciError::Transport)? {
            n if n == packet.len() => Ok(()),
            _ => Err(HciError::Transport(E_IO)),
        }
    }
    fn poll_event(&mut self) -> Polled {
        if let Some(p) = self.events.0.take()? {
            return Ok(Some(p));
        }
        let got = self.pipes.interrupt_in_poll(&mut self.events.1);
        take(&mut self.events, got)
    }
    fn poll_acl(&mut self) -> Polled {
        if let Some(p) = self.acl.0.take()? {
            return Ok(Some(p));
        }
        let got = self.pipes.bulk_in_poll(&mut self.acl.1);
        take(&mut self.acl, got)
    }
    fn pause(&mut self) {
        self.pipes.pause()
    }
}
