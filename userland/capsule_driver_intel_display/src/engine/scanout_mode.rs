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

//! A taken-over output's surfaces after a mode set, with the plane dark:
//! the old runs taken out of the GGTT, two new surfaces (contiguous IOVAs,
//! each already holding the picture) mapped inside the output's window of
//! the table (`window.0..window.1`, which other outputs never use) clear
//! of firmware's, and the plane pointed at the first, laid out as
//! firmware's was. The same call puts the old surfaces back when the new
//! mode could not be lit.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::flip::wait_live;
use super::ggtt;
use super::ggtt_range::pick;
use super::ggtt_run::map_run;
use super::plane::{program, SURF_ALIGN};
use super::scanout::Scanout;

impl Scanout {
    pub fn rebase(
        &mut self,
        m: &mut dyn Mmio,
        size: (u32, u32),
        window: (u32, u32),
        bases: [u64; 2],
        until: &dyn Expiry,
    ) -> Result<(), Error> {
        let stride = (size.0 * 4).next_multiple_of(64);
        let pages = (stride * size.1).div_ceil(4096);
        let total = window.1;
        for s in self.surfaces {
            ggtt::unmap(m, self.ggtt, s / 4096, self.pages, total)?;
        }
        let fw = self.firmware;
        let fw_pages = (fw.stride * fw.height).div_ceil(4096);
        let avoid = (fw.live / 4096, fw.live / 4096 + fw_pages);
        let align = SURF_ALIGN / 4096;
        let a = pick(total, pages, align, avoid).ok_or(Error::GgttFull)?;
        let b = pick(a, pages, align, avoid).filter(|&b| b >= window.0);
        let b = b.ok_or(Error::GgttFull)?;
        for (first, base) in [(a, bases[0]), (b, bases[1])] {
            map_run(m, self.ggtt, first, base, pages, total)?;
        }
        (self.width, self.height, self.stride) = (size.0, size.1, stride);
        (self.surfaces, self.pages, self.front) = ([a * 4096, b * 4096], pages, 0);
        program(m, fw.kind, self.pipe, a * 4096, stride, size, fw.ctl)?;
        wait_live(m, self.pipe, a * 4096, until)
    }
}
