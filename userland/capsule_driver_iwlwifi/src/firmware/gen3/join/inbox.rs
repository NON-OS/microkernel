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

//! Where the firmware's packets go while the station joins or is joined.
//! A received frame is parsed (`rx_data`) and kept, in order, for the join or
//! the link to read; a transmit response frees its queue's slots; a session
//! protection notification is remembered. The firmware may send any of these
//! while a host command waits for its reply, so they are routed the same way
//! then, and none is lost to the wait. The frame queue is bounded: a frame
//! arriving with it full is counted and dropped.

use alloc::collections::VecDeque;

use super::super::bringup::is_legacy;
use super::super::cmds::{MAC_CONF_GROUP, REPLY_RX_MPDU_CMD};
use super::super::packet::Packet;
use super::super::rx_data::{parse as parse_rx, RxMpdu};
use super::super::station::ids::{SESSION_PROTECTION_NOTIF, TX_CMD};
use super::super::station::session::{parse_notif, Session};
use super::super::tx_resp::parse as parse_tx_resp;
use super::super::txq::TxQueue;

/// Received frames held for the join or the link.
pub const INBOX_FRAMES: usize = 64;

/// The access point's two transmit queues.
pub struct Queues {
    pub mgmt: TxQueue,
    pub data: TxQueue,
}

#[derive(Default)]
pub struct Inbox {
    pub frames: VecDeque<RxMpdu>,
    /// The last session protection event not yet read.
    pub session: Option<Session>,
    /// Frames dropped with the queue full.
    pub dropped: u32,
    /// Transmit responses that named no queue of the access point's, or more
    /// slots than were in flight.
    pub stray: u32,
    /// Frames the firmware answered as not sent.
    pub tx_failed: u32,
}

impl Inbox {
    pub fn new() -> Self {
        Self::default()
    }

    /// Route one packet. `true` when it was a frame or an event for the
    /// reader (not a transmit response).
    pub fn route(&mut self, p: &Packet<'_>, q: &mut Queues) -> bool {
        if is_legacy(p, REPLY_RX_MPDU_CMD) {
            let Some(rx) = parse_rx(p.payload) else { return false };
            if self.frames.len() >= INBOX_FRAMES {
                self.dropped = self.dropped.saturating_add(1);
                return false;
            }
            self.frames.push_back(rx);
            return true;
        }
        if is_legacy(p, TX_CMD) {
            if let Some(s) = parse_tx_resp(p.payload) {
                let queue = if q.mgmt.live && s.queue == q.mgmt.id {
                    &mut q.mgmt
                } else if q.data.live && s.queue == q.data.id {
                    &mut q.data
                } else {
                    self.stray = self.stray.saturating_add(1);
                    return false;
                };
                if !s.sent {
                    self.tx_failed = self.tx_failed.saturating_add(1);
                }
                if !queue.reclaim(s.ssn) {
                    self.stray = self.stray.saturating_add(1);
                }
            }
            return false;
        }
        if p.group == MAC_CONF_GROUP && p.cmd == SESSION_PROTECTION_NOTIF {
            if let Some(s) = parse_notif(p.payload) {
                self.session = Some(s);
                return true;
            }
        }
        false
    }
}
