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

//! The bearer's ATT_MTU (Vol 3, Part F, 3.4.2). The host asks first,
//! offering `RX_MTU`. A server's answer from 23 up to that offer becomes
//! the ATT_MTU. Any other answer, an error, or a malformed response is
//! ignored, never trusted, and the default 23 holds. Every read the host
//! makes goes on while answers come full, so a server that sends longer
//! answers than its MTU said is still read whole.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::server::RX_MTU;
use nonos_bt_wire::att;

/// The LE default (Vol 3, Part G, 5.2.1), and the least a server may give.
pub const MTU_DEFAULT: u16 = 23;

impl Host {
    /// The bearer's ATT_MTU as this host uses it.
    pub fn mtu(&self) -> u16 {
        self.mtu
    }

    pub(super) fn exchange(&mut self, now_ms: u64) -> Step {
        self.phase = Phase::Mtu;
        self.request(att::exchange_mtu(RX_MTU), now_ms)
    }

    pub(super) fn on_mtu(&mut self, p: &[u8], now_ms: u64) -> Step {
        if let &[lo, hi] = p {
            let server_rx = u16::from_le_bytes([lo, hi]);
            if (MTU_DEFAULT..=RX_MTU).contains(&server_rx) {
                self.mtu = server_rx;
            }
        }
        self.find_services(1, now_ms)
    }
}
