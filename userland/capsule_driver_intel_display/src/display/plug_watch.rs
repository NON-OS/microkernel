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

//! Hot-plug watched from the takeover on: detection armed on DDIs B to D
//! and the lines' first reading kept, on Gen9 with an output that follows
//! its line (`plug`). Elsewhere nothing is armed or polled, so a family
//! whose hot-plug registers are not cited here is never written.

use super::frames::Frames;
use super::intel::Intel;
use crate::engine::access::{Clock, Mmio};
use crate::engine::gen::GEN9;
use crate::engine::hotplug::{arm, Poller};

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub fn watch(&mut self) {
        let follows = self.heads.iter().flatten().any(|h| h.modeset.is_some());
        if *self.gen != GEN9 || !follows {
            return;
        }
        arm(&mut self.m);
        let at = self.clock.now_us().unwrap_or(0);
        self.plugs = Some((Poller::new(&mut self.m), at));
    }
}
