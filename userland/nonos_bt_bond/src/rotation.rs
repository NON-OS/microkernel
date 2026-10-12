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

//! This host's resolvable private address, renewed on a timer. Vol 3,
//! Part C, Appendix A gives TGAP(private_addr_int) a recommended value of
//! 15 minutes; a new one is made from a fresh prand each time (Vol 6,
//! Part B, 1.3.2.2). Changing it is the caller's to do while it neither
//! scans nor connects (Vol 4, Part E, 7.8.4).

use crate::local::LocalIrk;
use nonos_bt_smp::privacy::{prand, rpa};
use nonos_bt_wire::addr::BdAddr;

pub const ROTATE_MS: u64 = 15 * 60 * 1000;

pub struct Rotation {
    irk: LocalIrk,
    current: Option<(BdAddr, u64)>,
}

impl Rotation {
    pub fn new(irk: LocalIrk) -> Self {
        Self { irk, current: None }
    }

    /// Whether the address has served its time, or none was made yet.
    pub fn due(&self, now_ms: u64) -> bool {
        self.current.is_none_or(|(_, made)| now_ms.saturating_sub(made) >= ROTATE_MS)
    }

    /// A new address from 24 drawn bits; None when the draw is one 1.3.2.2
    /// forbids, and the caller draws again.
    pub fn renew(&mut self, draw: [u8; 3], now_ms: u64) -> Option<BdAddr> {
        let addr = rpa(self.irk.key(), &prand(draw)?);
        self.current = Some((addr, now_ms));
        Some(addr)
    }

    pub fn current(&self) -> Option<BdAddr> {
        self.current.map(|(a, _)| a)
    }

    pub fn irk(&self) -> &LocalIrk {
        &self.irk
    }
}
