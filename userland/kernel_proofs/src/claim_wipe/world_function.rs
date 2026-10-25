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

//! The function a seed draws for the reset method under test: what it offers
//! read through the kernel's own `offers` (whose `method` must then be the
//! one under test) and its reach past Bus Master through
//! `binds_past_bus_master`, beside what it truly is and how it misbehaves.

use super::bits_rule::{AF_FLR_CAP, AF_TP_CAP, FLR_CAP, NO_SOFT_RESET};
use super::method_rule::{binds_past_bus_master, method, offers, Method};
use super::world::World;
use super::world_dev::{Caps, Dev};

/// (untranslated, vendor, class): VMMDev, an Intel display, another display
/// and two plain functions, the last drawn half the time.
const KINDS: [(bool, u16, u8); 5] = [
    (true, 0x80EE, 8),
    (false, 0x8086, 3),
    (false, 0x1234, 3),
    (false, 0x10EC, 2),
    (false, 0x8086, 2),
];

impl World {
    pub(super) fn function(&mut self, m: Option<Method>) -> Dev {
        let (flr, af) = (m == Some(Method::Flr), m == Some(Method::AfFlr));
        let low = self.draw(1 << 16) as u32;
        let devcap = (flr || self.chance(2)).then_some(low | if flr { FLR_CAP } else { 0 });
        let tp = self.draw(2) as u8 * AF_TP_CAP;
        let af_caps =
            (af || (!flr && self.chance(4))).then_some(tp | if af { AF_FLR_CAP } else { 0 });
        let soft = m == Some(Method::PmCycle) || (m.is_some() && self.chance(2));
        let pmcsr = (soft || self.chance(2)).then_some(if soft { 0 } else { NO_SOFT_RESET });
        let offers = offers(devcap, af_caps, pmcsr);
        assert_eq!(method(offers), m, "the function drawn offers the method under test");
        let (untranslated, vendor, class) = KINDS[self.draw(8).min(4) as usize];
        let device = 0x1000 + self.draw(0xE000) as u32;
        let caps = Caps {
            offers,
            pcie: devcap.is_some(),
            af_tp: af_caps.is_some_and(|c| c & AF_TP_CAP != 0),
            pm: pmcsr.is_some(),
            ident: u32::from(vendor) | (device << 16),
            display: class == 3,
            binds: binds_past_bus_master(untranslated, vendor, class),
            untranslated,
            mem_bar: !self.chance(5),
        };
        let (ignores_flr, ignores_d3, stuck_bm) = (self.chance(6), self.chance(12), self.chance(6));
        let (intx_sticks, retry, tp_never) = (!self.chance(5), self.chance(2), self.chance(10));
        // VMMDev, a fact in the tree; an Intel display (recalled, unverified).
        let past_bm = untranslated || (vendor, class) == (0x8086, 3);
        let (flr, af_flr, soft_d3) = (flr, af, soft);
        let truth = Dev { flr, af_flr, soft_d3, past_bm, ..Dev::default() };
        let quirks =
            Dev { ignores_flr, ignores_d3, stuck_bm, intx_sticks, retry, tp_never, ..truth };
        Dev { caps, ..quirks }
    }
}
