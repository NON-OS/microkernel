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

use alloc::vec::Vec;

use crate::crypto::Key;
use crate::packet::REPLAY_TAG_LEN;
use crate::surb::Budget;

use super::gateway::Gateway;
use super::replay::ReplayWindow;
use super::rx_queue::RxQueue;

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
    replay: ReplayWindow,
    rx: RxQueue,
}

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
            replay: ReplayWindow::new(),
            rx: RxQueue::new(),
        }
    }

    pub fn push(&mut self, body: Vec<u8>) {
        let _ = self.rx.push(body);
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        self.rx.take(usize::MAX)
    }

    /// Messages and bytes waiting for the reader.
    pub fn backlog(&self) -> (usize, usize) {
        (self.rx.len(), self.rx.bytes())
    }

    /// Take at most `limit` bytes of the next message, leaving the rest where
    /// a later read will find it. See `RxQueue::take`.
    pub fn take(&mut self, limit: usize) -> Option<Vec<u8>> {
        self.rx.take(limit)
    }

    /// Move as many queued messages as fit into `out`, as records. See
    /// `RxQueue::fill_records`.
    pub fn fill_records(&mut self, out: &mut [u8]) -> usize {
        self.rx.fill_records(out)
    }

    pub fn accept_replay_tag(&mut self, tag: &[u8; REPLAY_TAG_LEN]) -> bool {
        self.replay.accept(tag)
    }

    pub fn zeroize(&mut self) {
        self.key.fill(0);
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
