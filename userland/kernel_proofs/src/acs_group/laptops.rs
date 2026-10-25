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

//! The machines of A's K11 facts letter as the read layer sees them: each
//! function's ids, PCIe type and ACS registers, read through the real
//! layout choice and word reading.

use std::cell::RefCell;

use super::acs_rule::{CR, PORT, RR, SV};
use super::acs_tree::{Bdf, Func};
use super::words_tests::Regs;

pub(super) struct Part {
    pub func: Func,
    pub ids: (u16, u16),
    pub kind: Option<u8>,
    pub acs: Option<Regs>,
}

pub(super) const WIFI: Bdf = (1, 0, 0);
pub(super) const NVME: Bdf = (2, 0, 0);
pub(super) const RP0: Bdf = (0, 0x1c, 0);
pub(super) const RP4: Bdf = (0, 0x1c, 4);

pub(super) fn part(at: Bdf, buses: Option<(u8, u8)>, ids: (u16, u16), kind: Option<u8>) -> Part {
    let func = Func { at, buses, conventional: false, alias: None };
    Part { func, ids, kind, acs: None }
}

fn regs(cap: u16, ro_cap: bool) -> Option<Regs> {
    Some(Regs { r: RefCell::new([0, u32::from(cap), 0]), ro_cap })
}

/// Wi-Fi under 1c.0 and NVMe under 1c.4, root ports of `ids` with `acs`.
pub(super) fn laptop(ids: [u16; 2], acs: impl Fn() -> Option<Regs>) -> Vec<Part> {
    let mut rp0 = part(RP0, Some((1, 1)), (0x8086, ids[0]), Some(4));
    let mut rp4 = part(RP4, Some((2, 2)), (0x8086, ids[1]), Some(4));
    (rp0.acs, rp4.acs) = (acs(), acs());
    let wifi = part(WIFI, None, (0x10EC, 0xC821), Some(0));
    vec![rp0, rp4, wifi, part(NVME, None, (0x144D, 0xA808), Some(0))]
}

pub(super) fn spt() -> Vec<Part> {
    laptop([0xA110, 0xA114], || regs(PORT, true))
}

/// Cannon Point: standard ACS, no Upstream Forwarding offered.
pub(super) fn cnp() -> Vec<Part> {
    laptop([0x9DB8, 0x9DBC], || regs(SV | RR | CR, false))
}

pub(super) fn pre_spt() -> Vec<Part> {
    laptop([0x8C10, 0x8C18], || None)
}
