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

//! A request the checks let through: the guest info kept, the host's
//! version (7.0.20) answered, the filter and capabilities masked in and
//! out, the mouse features set or read, the pending events handed over
//! and cleared.

use super::host::Host;

const VERR_ACCESS_DENIED: i32 = -38;

impl Host {
    pub(super) fn carry_out(&mut self, kind: u32, b: &[u32], p: *mut u32) -> i32 {
        // SAFETY: the body words written are inside the size checked.
        let out = |i: usize, v: u32| unsafe { *p.add(6 + i) = v.to_le() };
        match kind {
            50 if self.opt.refuse_info => return VERR_ACCESS_DENIED,
            50 => self.info = Some((b[0], b[1])),
            4 => [7, 20, 166_000, 0].iter().enumerate().for_each(|(i, &v)| out(i, v)),
            42 => self.filter = (self.filter | b[0]) & !b[1],
            56 => self.caps = (self.caps | b[0]) & !b[1],
            2 => self.guest_mouse = b[0] & 0x15,
            1 => {
                let features = self.guest_mouse | if self.integration { 1 << 1 } else { 0 };
                [features, self.at.0 as u32, self.at.1 as u32]
                    .iter()
                    .enumerate()
                    .for_each(|(i, &v)| out(i, v));
            }
            41 => {
                let events = self.pending & self.filter;
                self.pending &= !events;
                self.raise();
                out(0, events);
            }
            _ => unreachable!(),
        }
        0
    }
}
