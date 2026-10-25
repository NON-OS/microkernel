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

//! The owner's silence. The compositor asks STATE of each driver it
//! drives every 500 ms; one silent for OWNER_SILENCE_MS has died or hung,
//! and what it last composed (a client's pixels, a password field among
//! them) must not stay on the screen. Each output's cursor is turned off,
//! every primary scrubbed, and each output shown from its scrubbed primary
//! twice, so both of a double-buffered device's frames are blank. The owner
//! keeps its place: one that was only slow is heard again and its next
//! STATE says LOST, so it repaints whole; a restarted one asks SURFACE.

use nonos_gfx_wire::{Cursor, Rect};

use super::display::Display;
use super::platform::Platform;
use super::server::Server;
use super::watch_tell::Tell;

/// Six of the compositor's polls missed.
pub const OWNER_SILENCE_MS: u64 = 3_000;

impl<D: Display, P: Platform> Server<D, P> {
    /// A request from `sender` came at `now_ms`.
    pub fn heard(&mut self, sender: u32, now_ms: u64) {
        if self.owner.is_none_or(|o| o == sender) {
            (self.heard_ms, self.quiet, self.tell) = (now_ms, false, Tell::Quiet);
        }
    }

    /// At `now_ms`: an owner silent too long has the outputs blanked, once
    /// for each silence the devices took whole; true when it ran now.
    pub fn watch(&mut self, now_ms: u64) -> bool {
        let silent = now_ms.saturating_sub(self.heard_ms) >= OWNER_SILENCE_MS;
        if self.owner.is_none() || self.quiet || !silent {
            return false;
        }
        let mut whole = true;
        for o in 0..self.slots.len() {
            let slot = self.slots[o];
            for p in [slot.current, slot.retiring].into_iter().flatten() {
                self.platform.scrub(p.region);
            }
            let Some(c) = slot.current else { continue };
            let off = Cursor { flags: 0, x: 0, y: 0, hot_x: 0, hot_y: 0, width: 0, height: 0 };
            let all = [Rect { x: 0, y: 0, width: c.width, height: c.height }];
            whole &= self.with_shadow(c, |d, s| {
                let _ = d.cursor(o as u16, &off, &[]);
                d.flush(o as u16, &all, s).is_ok() & d.flush(o as u16, &all, s).is_ok()
            });
            self.slots[o].blanked = true;
        }
        // A blank a device did not take whole is tried again next wake.
        self.quiet = whole;
        self.tell.ran();
        true
    }
}
