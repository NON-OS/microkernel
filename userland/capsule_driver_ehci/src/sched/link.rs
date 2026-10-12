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
//! How a device is reached: its speed (the host wire's ids: 1 full, 2 low,
//! 3 high) and, below a high-speed hub, the TT its split transactions go
//! through (hub address, port). A QH's speed, control flag and split
//! fields follow from it. Pure.

use crate::facts::qh::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Link {
    pub speed: u8,
    pub tt: Option<(u8, u8)>,
}

impl Link {
    /// A high-speed device: on a root port or below high-speed hubs.
    pub const HIGH: Link = Link { speed: 3, tt: None };

    pub const fn high(self) -> bool {
        self.speed == 3
    }

    /// Characteristics `ch` (built for high speed) set for this speed; a
    /// full- or low-speed control endpoint is marked as one.
    pub const fn chars(self, ch: u32, control: bool) -> u32 {
        let eps = match self.speed {
            1 => CH_EPS_FULL,
            2 => CH_EPS_LOW,
            _ => CH_EPS_HIGH,
        };
        let c = if control && !self.high() { CH_C } else { 0 };
        (ch & !CH_EPS_MASK) | eps | c
    }

    /// Capabilities: the TT when split, and for a periodic QH micro-frame
    /// 0's start (with completes after it when split).
    pub const fn caps(self, periodic: bool) -> u32 {
        let tt = match self.tt {
            Some((hub, port)) if !self.high() => {
                (hub as u32 & 0x7F) << CAP_HUB_SHIFT | (port as u32 & 0x7F) << CAP_PORT_SHIFT
            }
            _ => 0,
        };
        let mask = match (periodic, tt != 0) {
            (false, _) => 0,
            (true, false) => CAP_SMASK_UFRAME0,
            (true, true) => CAP_SMASK_UFRAME0 | CAP_CMASK_SPLIT,
        };
        CAP_MULT_1 | tt | mask
    }
}

impl Default for Link {
    fn default() -> Self {
        Self::HIGH
    }
}
