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

//! A client's session: where it sends, the keys it seals under, and what
//! waits for it to read.

use crate::crypto::Key;
use crate::surb::Budget;

use super::super::gateway::Gateway;
use super::super::rx_queue::RxQueue;

pub struct Session {
    pub owner: u32,
    pub id: u32,
    pub gateway: Gateway,
    pub key: Key,
    /// Zero until set. Without one there is nowhere to route, so the session
    /// cannot be sealed as Sphinx.
    pub dest: [u8; 32],
    /// The exit's x25519 key. A message is sealed for the exit alone under a
    /// key agreed against this, so the identity above cannot stand in for it:
    /// one names the destination, the other encrypts to it.
    pub dest_encryption: [u8; 32],
    /// The gateway the recipient holds its session with. Only that node can
    /// hand a packet to it, so the route out has to end there.
    pub dest_gateway: [u8; 32],
    pub dest_id: [u8; 16],
    /// What an exit quotes to reach us again. It is ours and it is random:
    /// the exit never learns anything else about where we are, and a tag
    /// derived from something it already knows would tell it more.
    pub sender_tag: [u8; 16],
    /// Reply blocks the far end is believed to hold under that tag.
    pub surbs: Budget,
    pub(super) rx: RxQueue,
}
