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

//! The link net_core sees when the radio never came up.

use nonos_wifi_core::netif::LinkPort;

/// The link net_core sees when the radio never came up. It reports down, holds no
/// MAC, and refuses traffic, so the driver can keep answering (and reporting how
/// far bring-up got) instead of exiting and taking its service registration with
/// it. It touches no hardware, which is why it is safe when the chip is dead.
pub struct DeadLink;

impl LinkPort for DeadLink {
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
