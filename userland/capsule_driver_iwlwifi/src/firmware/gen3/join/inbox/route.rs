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
//! Each packet the firmware posts, routed to where it is read.

use super::super::super::bringup::is_legacy;
use super::super::super::cmds::{MAC_CONF_GROUP, REPLY_RX_MPDU_CMD};
use super::super::super::packet::Packet;
use super::super::super::rx_data::parse as parse_rx;
use super::super::super::station::ids::{SESSION_PROTECTION_NOTIF, TX_CMD};
use super::super::super::station::session::parse_notif;
use super::super::super::tx_resp::parse as parse_tx_resp;
use super::missed::{quiet, MISSED_BEACONS_NOTIF};
use super::state::{Inbox, Queues};
use super::INBOX_FRAMES;

impl Inbox {
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
        if p.group == MAC_CONF_GROUP && p.cmd == MISSED_BEACONS_NOTIF {
            self.missed |= quiet(p.payload);
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
