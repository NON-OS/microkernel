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

//! Taking an output over: both surfaces mapped high in the GGTT, the plane
//! pointed at the first, and nothing returned until it is live.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::flip::wait_live;
use super::ggtt;
use super::ggtt_range::pick;
use super::plane::{program, SURF_ALIGN};
use super::scanout::Scanout;
use super::takeover::Output;

impl Scanout {
    /// Map both surfaces (`pages` IOVAs each, the first already holding
    /// the picture) high in the GGTT, clear of firmware's surface, then
    /// point the plane at the first and wait until it is live.
    pub fn start(
        m: &mut dyn Mmio,
        out: &Output,
        total: u32,
        iovas: [&[u64]; 2],
        until: &dyn Expiry,
    ) -> Result<Scanout, Error> {
        let fw = out.plane;
        let stride = (out.width * 4).next_multiple_of(64);
        let pages = (stride * out.height).div_ceil(4096);
        if !fw.flippable() || fw.width != out.width || fw.height != out.height {
            return Err(Error::Layout);
        }
        if iovas.iter().any(|v| v.len() != pages as usize) {
            return Err(Error::Argument);
        }
        let align = SURF_ALIGN / 4096;
        let fw_first = fw.live / 4096;
        let fw_pages = (fw.stride.max(stride) * fw.height).div_ceil(4096);
        let avoid = (fw_first, fw_first + fw_pages);
        let a = pick(total, pages, align, avoid).ok_or(Error::GgttFull)?;
        let b = pick(a, pages, align, avoid).ok_or(Error::GgttFull)?;
        ggtt::map(m, out.ggtt, a, iovas[0], total)?;
        ggtt::map(m, out.ggtt, b, iovas[1], total)?;
        let surfaces = [a * 4096, b * 4096];
        program(m, fw.kind, out.pipe, surfaces[0], stride, (out.width, out.height), fw.ctl)?;
        wait_live(m, out.pipe, surfaces[0], until)?;
        let (width, height) = (out.width, out.height);
        Ok(Scanout {
            pipe: out.pipe,
            width,
            height,
            stride,
            surfaces,
            pages,
            ggtt: out.ggtt,
            front: 0,
            firmware: fw,
        })
    }
}
