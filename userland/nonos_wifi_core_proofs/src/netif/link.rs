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

//! A modeled link for the net_core adapter proofs: it reports a MAC and a
//! link state, queues frames to receive, and records the frames it is sent.

use core::cell::RefCell;
use nonos_wifi_core::netif::serve::LinkPort;

pub struct MockLink {
    pub mac: Option<[u8; 6]>,
    pub up: bool,
    pub rx: RefCell<Vec<Vec<u8>>>,
    pub sent: RefCell<Vec<Vec<u8>>>,
}

impl MockLink {
    pub fn up_with(mac: [u8; 6]) -> Self {
        Self {
            mac: Some(mac),
            up: true,
            rx: RefCell::new(Vec::new()),
            sent: RefCell::new(Vec::new()),
        }
    }
    pub fn down() -> Self {
        Self { mac: None, up: false, rx: RefCell::new(Vec::new()), sent: RefCell::new(Vec::new()) }
    }
}

impl LinkPort for MockLink {
    fn mac(&self) -> Option<[u8; 6]> {
        self.mac
    }
    fn link_up(&self) -> bool {
        self.up
    }
    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize> {
        let mut rx = self.rx.borrow_mut();
        if rx.is_empty() {
            return None;
        }
        let f = rx.remove(0);
        out[..f.len()].copy_from_slice(&f);
        Some(f.len())
    }
    fn send_tx(&mut self, frame: &[u8]) -> bool {
        if !self.up {
            return false;
        }
        self.sent.borrow_mut().push(frame.to_vec());
        true
    }
}
