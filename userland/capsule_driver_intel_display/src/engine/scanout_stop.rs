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

//! Handing an output back: firmware's plane registers are put back and
//! latched before the driver's GGTT entries go, so the engine never
//! scans out an entry that is no longer present.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::flip::wait_live;
use super::ggtt;
use super::plane_regs as r;
use super::scanout::Scanout;

impl Scanout {
    pub fn stop(self, m: &mut dyn Mmio, total: u32, until: &dyn Expiry) -> Result<(), Error> {
        let fw = self.firmware;
        let p = self.pipe;
        m.write(r::plane_stride(p, 1), fw.stride / 64);
        m.write(r::plane_size(p, 1), ((fw.height - 1) << 16) | (fw.width - 1));
        m.write(r::plane_ctl(p, 1), fw.ctl);
        m.write(r::plane_surf(p, 1), fw.live);
        wait_live(m, p, fw.live, until)?;
        for s in self.surfaces {
            ggtt::unmap(m, s / 4096, self.pages, total)?;
        }
        Ok(())
    }
}
