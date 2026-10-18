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

//! Hot-plug detection armed on the south display's DDIs B to D (SKL PRM
//! Vol 2c, SHOTPLUG_CTL; the offset and enable bits recalled, unverified):
//! firmware arms only the ports it lit, and a line not armed reads low
//! whatever is plugged in. The status bits beside each enable are write-1
//! to clear; writing back what was read clears only events this driver
//! never reads, since it polls the live lines (`hotplug`).

use super::super::access::{rmw, Mmio};

pub const SHOTPLUG_CTL: u32 = 0xC_4030;
/// DDI B's, C's and D's detect enables.
pub const ENABLES: u32 = (1 << 4) | (1 << 12) | (1 << 20);

pub fn arm(m: &mut dyn Mmio) {
    rmw(m, SHOTPLUG_CTL, ENABLES, ENABLES);
}
