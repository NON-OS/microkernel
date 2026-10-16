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

//! The Three-wire UART on one port, receiving: octets in, through SLIP and
//! the packet checks, to link control, the reliable layer and the HCI
//! packets they carry. Before the link is Active only link control is
//! read, since nothing else can be numbered yet. A packet whose header or
//! check does not hold is counted in `bad` and dropped.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::control::parse;
use super::deslip::Deslip;
use super::header::{COMMAND, EVENT, LINK_CONTROL};
use super::link::{Link, State};
use super::packet::open;
use super::reliable::Reliable;

#[derive(Default)]
pub struct H5 {
    pub(super) deslip: Deslip,
    pub(super) link: Link,
    pub(super) rel: Reliable,
    /// Kind and packet of each HCI packet taken, oldest first: commands,
    /// ACL, synchronous data and events, for whichever side this is.
    pub inbox: VecDeque<(u8, Vec<u8>)>,
    pub(super) replies: VecDeque<Vec<u8>>,
    pub(super) peer_asleep: bool,
    pub(super) woke_at: Option<u64>,
    pub bad: u32,
    /// Times the peer started the link over.
    pub restarts: u32,
}

impl H5 {
    pub fn active(&self) -> bool {
        self.link.state == State::Active
    }

    pub fn feed(&mut self, octets: &[u8], now: u64) {
        for &b in octets {
            if let Some(frame) = self.deslip.push(b) {
                self.take_frame(&frame, now);
            }
        }
    }

    fn take_frame(&mut self, f: &[u8], now: u64) {
        let Ok((h, payload)) = open(f) else { return self.bad = self.bad.saturating_add(1) };
        if h.kind == LINK_CONTROL && !h.reliable {
            return parse(payload).into_iter().for_each(|m| self.control(m));
        }
        if !self.active() {
            return;
        }
        self.rel.acked(h.ack, now);
        if h.reliable && self.rel.received(h.seq) && (COMMAND..=EVENT).contains(&h.kind) {
            self.inbox.push_back((h.kind, payload.to_vec()));
        }
    }
}
