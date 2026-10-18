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

//! What the host core needs from a transport: whole HCI packets each way,
//! with no packet indicator (the USB transport has none, Vol 4, Part B, 2;
//! a UART transport would add and strip it below this trait).

use alloc::vec::Vec;

use crate::error::HciError;

pub trait Transport {
    /// Send one command packet.
    fn send_command(&mut self, packet: &[u8]) -> Result<(), HciError>;
    /// Send one ACL data packet.
    fn send_acl(&mut self, packet: &[u8]) -> Result<(), HciError>;
    /// The next whole event packet, `None` while none has come. Never
    /// waits: the caller bounds its own waiting with `pause`.
    fn poll_event(&mut self) -> Result<Option<Vec<u8>>, HciError>;
    /// The next whole ACL data packet, `None` while none has come.
    fn poll_acl(&mut self) -> Result<Option<Vec<u8>>, HciError>;
    /// Give the controller time between two empty polls; a capsule sleeps
    /// about a millisecond, the proofs' simulated controller not at all.
    fn pause(&mut self);
}
