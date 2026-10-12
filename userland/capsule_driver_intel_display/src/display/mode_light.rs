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

//! One lighting of a taken-over output at `timing` on the surfaces at
//! `bases`: the port relit, the plane's buffer and watermarks for the new
//! line, then the plane rebased onto the surfaces, live before it returns.

use super::frames::Frames;
use super::intel::{Intel, FLIP_US};
use super::modeset::Modeset;
use crate::engine::access::{Clock, Mmio, Until};
use crate::engine::error::Error;
use crate::engine::relight::{buffer, relight};
use nonos_edid::Detailed;

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub(super) fn light(
        &mut self,
        ms: &Modeset,
        timing: &Detailed,
        size: (u32, u32),
        bases: [u64; 2],
    ) -> Result<(), Error> {
        relight(&mut self.m, &self.clock, &ms.lit, timing, ms.tmds_khz)?;
        buffer(&mut self.m, ms.lit.p, size.0, timing, &ms.latency_us);
        let until = Until::new(&self.clock, FLIP_US);
        self.scan.rebase(&mut self.m, size, ms.total, bases, &until)
    }
}
