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

//! What the takeover model asks of a world (L79): the console's stop as
//! the kernel reads it, whether the drawer writes, whether a running bootfb
//! page shows the framebuffer, and whether a covering claim is ready.

use super::types::Stage;
use super::world::{code, END};
use super::world_state::World;

impl World {
    /// held::stopped.
    pub(super) fn stopped(&self) -> bool {
        self.held || self.gone || self.takers > 0
    }
    /// K between its re-check and the end of its write.
    pub(super) fn drawing(&self) -> bool {
        self.pc[5] < END && self.pc[5] % 4 == 3
    }
    /// A page of a bootfb thread still running shows the framebuffer.
    pub(super) fn shown(&self) -> bool {
        !self.dead && self.page.iter().any(|p| p.shows_fb())
    }
    pub(super) fn ready(&self) -> bool {
        self.gpu.contains(&Some(code(Stage::Ready)))
    }
}
