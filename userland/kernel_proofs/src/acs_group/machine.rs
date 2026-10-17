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

//! A machine for the tests: a root port on bus 0 with a switch below it,
//! two endpoints under the switch, and a multi-function device on bus 0.

use super::acs_tree::{Bdf, Func};

pub(super) const ROOT_PORT: Bdf = (0, 0x1c, 0);
pub(super) const ROOT_PORT_2: Bdf = (0, 0x1c, 4);
pub(super) const UPSTREAM: Bdf = (1, 0, 0);
pub(super) const DOWN_A: Bdf = (2, 1, 0);
pub(super) const DOWN_B: Bdf = (2, 2, 0);
pub(super) const NIC: Bdf = (3, 0, 0);
pub(super) const WIFI: Bdf = (4, 0, 0);
pub(super) const SATA: Bdf = (0, 0x1f, 2);
pub(super) const LPC: Bdf = (0, 0x1f, 0);
pub(super) const NVME: Bdf = (5, 0, 0);

fn bridge(at: Bdf, sec: u8, sub: u8) -> Func {
    Func { at, buses: Some((sec, sub)), conventional: false, alias: None }
}

fn leaf(at: Bdf) -> Func {
    Func { at, buses: None, conventional: false, alias: None }
}

pub(super) fn machine() -> Vec<Func> {
    vec![
        bridge(ROOT_PORT, 1, 4),
        bridge(ROOT_PORT_2, 5, 5),
        bridge(UPSTREAM, 2, 4),
        bridge(DOWN_A, 3, 3),
        bridge(DOWN_B, 4, 4),
        leaf(NIC),
        leaf(WIFI),
        leaf(SATA),
        leaf(LPC),
        leaf(NVME),
    ]
}
