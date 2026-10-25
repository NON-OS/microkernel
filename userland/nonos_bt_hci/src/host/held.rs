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

//! The ACL buffers each link holds (Vol 4, Part E, 4.1.1): one per packet
//! sent on it, given back by Number of Completed Packets (7.7.19) for that
//! link and never past what it holds, and all at once when it ends, its
//! unsent packets flushed with it (7.7.5).

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::event::parse_event;
use nonos_bt_wire::hci::event_code::{DISCONNECTION_COMPLETE, NUMBER_OF_COMPLETED_PACKETS};
use nonos_bt_wire::hci::reports::completed_packets;
use nonos_bt_wire::hci::DisconnectionComplete;

impl<T: Transport> Host<T> {
    pub(super) fn hold(&mut self, handle: u16) {
        self.acl_free -= 1;
        match self.held.iter_mut().find(|(h, _)| *h == handle) {
            Some((_, n)) => *n += 1,
            None => self.held.push((handle, 1)),
        }
    }

    /// Up to `n` of the buffers `handle` holds given back.
    fn give_back(&mut self, handle: u16, n: u16) {
        let Some(i) = self.held.iter().position(|&(h, _)| h == handle) else { return };
        let back = n.min(self.held[i].1);
        self.held[i].1 -= back;
        if self.held[i].1 == 0 {
            self.held.swap_remove(i);
        }
        self.acl_free = self.acl_free.saturating_add(back).min(self.acl_total);
    }

    /// Every buffer of a link Disconnect found already gone.
    pub fn link_gone(&mut self, handle: u16) {
        self.give_back(handle, u16::MAX);
    }

    /// What one event says of the buffers and of the controller; `true`
    /// for Number of Completed Packets, which the caller never sees.
    pub(super) fn note(&mut self, event: &[u8]) -> Result<bool, HciError> {
        self.lost(event)?;
        let ev = parse_event(event)?;
        if ev.code == DISCONNECTION_COMPLETE {
            if let Some(d) = DisconnectionComplete::parse(ev.params).ok().filter(|d| d.status == 0)
            {
                self.link_gone(d.handle);
            }
            return Ok(false);
        }
        if ev.code == NUMBER_OF_COMPLETED_PACKETS {
            for (handle, n) in completed_packets(ev.params)? {
                self.give_back(handle, n);
            }
        }
        Ok(ev.code == NUMBER_OF_COMPLETED_PACKETS)
    }
}
