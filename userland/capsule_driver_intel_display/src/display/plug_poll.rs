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

//! The hot-plug poll (NONOS takes no interrupts; `plug_watch` starts it):
//! every `POLL_US` the serve loop asks, each DDI's line is read, and a
//! change two polls agree on (`hotplug::Poller`) moves the output on that
//! DDI. Only a settled pull darkens one, so a line read low from the
//! start, or one firmware never armed, never darkens an output showing a
//! picture. A line settled high under a dark output has its monitor read
//! again on every poll until it answers, since a monitor just plugged in
//! may still be waking.

use super::frames::Frames;
use super::intel::{Intel, MAX_HEADS};
use super::plug::Plug;
use crate::engine::access::{Clock, Mmio};

/// A tenth of a second between polls: a plug settles within two, and a
/// bounce shorter than one poll is never seen.
pub const POLL_US: u64 = 100_000;

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    /// One poll once `POLL_US` have passed since the last; the serve loop
    /// calls it every turn, after a frame or its receive's timeout. Whether
    /// an output went dark or came back.
    pub fn tick(&mut self) -> bool {
        let (Some(now), Some((_, at))) = (self.clock.now_us(), self.plugs.as_mut()) else {
            return false;
        };
        let due = now.wrapping_sub(*at) >= POLL_US;
        if due {
            *at = now;
        }
        let moved = due && self.poll_plugs();
        if due {
            self.poll_caps();
        }
        moved
    }

    /// One poll of the lines, unpaced; whether any output moved.
    pub fn poll_plugs(&mut self) -> bool {
        let Some((p, _)) = self.plugs.as_mut() else { return false };
        let was = self.heads.each_ref().map(|h| h.as_ref().map(|h| h.plug));
        let (settled, known) = (p.poll(&mut self.m), p.known);
        for o in 0..MAX_HEADS {
            let h = self.heads[o].as_ref();
            let Some((ddi, plug)) = h.and_then(|h| Some((h.modeset?.lit.ddi as usize, h.plug)))
            else {
                continue;
            };
            let (Some(&now), Some(&line)) = (settled.get(ddi), known.get(ddi)) else { continue };
            match plug {
                Plug::In | Plug::Back if now == Some(false) => self.pulled(o),
                Plug::Out if line => self.replugged(o),
                _ => {}
            }
        }
        was != self.heads.each_ref().map(|h| h.as_ref().map(|h| h.plug))
    }
}
