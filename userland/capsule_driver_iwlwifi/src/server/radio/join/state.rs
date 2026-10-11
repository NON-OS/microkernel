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
//! The radio's join state, and the link net_core sees with no port open.
use crate::firmware::gen3::join::inbox::{Inbox, Queues};
use crate::firmware::gen3::join::run::Joined;
use crate::firmware::gen3::txq::{TxQueue, TXQ_STRIDE};
use nonos_wifi_core::netif::LinkPort;

use super::super::grant::Grant;

pub(super) const SSID_MAX: usize = 32;

/// The radio's join state.
pub struct Join {
    pub(super) tx: Option<&'static Grant>,
    pub(super) q: Queues,
    pub(super) inbox: Inbox,
    pub(super) joined: Option<Joined>,
    pub(super) ssid: [u8; SSID_MAX],
    pub(super) ssid_len: usize,
}

impl Join {
    pub fn new() -> Self {
        Self {
            tx: None,
            q: Queues { mgmt: TxQueue::new(0), data: TxQueue::new(TXQ_STRIDE) },
            inbox: Inbox::new(),
            joined: None,
            ssid: [0; SSID_MAX],
            ssid_len: 0,
        }
    }
}
/// net_core's view of a radio with no open port.
pub(super) struct Down;

impl LinkPort for Down {
    fn mac(&self) -> Option<[u8; 6]> {
        None
    }
    fn link_up(&self) -> bool {
        false
    }
    fn poll_rx(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
    fn send_tx(&mut self, _frame: &[u8]) -> bool {
        false
    }
}
