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

//! Opening a session, with a sender tag of its own.

use crate::crypto::Key;
use crate::surb::Budget;

use super::super::gateway::Gateway;
use super::super::rx_queue::RxQueue;
use super::types::Session;

impl Session {
    pub fn new(owner: u32, id: u32, gateway: Gateway, key: Key) -> Self {
        Self {
            owner,
            id,
            gateway,
            key,
            dest: [0u8; 32],
            dest_encryption: [0u8; 32],
            dest_gateway: [0u8; 32],
            dest_id: [0u8; 16],
            sender_tag: random_tag(),
            surbs: Budget::new(),
            rx: RxQueue::new(),
        }
    }
}

/// A fresh sender tag, or zeros if there was no entropy to draw one from.
///
/// Zeros are left deliberately recognisable rather than replaced with
/// anything derived. Every session without entropy would otherwise share a
/// tag, which is the link the tag exists to avoid, so the send path treats
/// an unset tag as a reason to refuse rather than something to send.
fn random_tag() -> [u8; 16] {
    let mut tag = [0u8; 16];
    if crate::crypto::random::fill_random(&mut tag).is_err() {
        return [0u8; 16];
    }
    tag
}
