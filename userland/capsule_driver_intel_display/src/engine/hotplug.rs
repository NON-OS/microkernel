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

//! Hot-plug by polling: the live connection state of each DDI's hot-plug
//! line (SKL PRM Vol 2c, SDEISR for DDIs B to D in the south display,
//! GEN8_DE_PORT_ISR for DDI A; bit positions unverified), with a change
//! reported only once two polls agree, so a plug's bounce is not two
//! events. DDI A is usually the eDP panel, which is always there.

use super::access::Mmio;

pub const SDEISR: u32 = 0xC_4000;
pub const DE_PORT_ISR: u32 = 0x4_4440;

/// Each DDI's line, A to D.
pub fn live(m: &mut dyn Mmio) -> [bool; 4] {
    let south = m.read(SDEISR);
    let north = m.read(DE_PORT_ISR);
    [north & (1 << 3) != 0, south & (1 << 21) != 0, south & (1 << 22) != 0, south & (1 << 23) != 0]
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Poller {
    pub known: [bool; 4],
    seen: [bool; 4],
}

impl Poller {
    /// Start from the lines as they are now.
    pub fn new(m: &mut dyn Mmio) -> Self {
        let l = live(m);
        Poller { known: l, seen: l }
    }

    /// One poll: the DDIs whose state settled to something new since the
    /// last report, with that state.
    pub fn poll(&mut self, m: &mut dyn Mmio) -> [Option<bool>; 4] {
        let now = live(m);
        let mut out = [None; 4];
        for i in 0..4 {
            if now[i] == self.seen[i] && now[i] != self.known[i] {
                self.known[i] = now[i];
                out[i] = Some(now[i]);
            }
            self.seen[i] = now[i];
        }
        out
    }
}
