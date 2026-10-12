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

//! One bring-up on the boot framebuffer's record, in the order the kernel
//! asks (lead L31): driver.bootfb0 announced first (only its server may
//! claim), the claim, the mode (holder only), then bar 0 mapped as the
//! kernel maps it; any step that fails releases the claim, which takes a
//! mapping already made and hands the screen back to the kernel.

use super::fb::Fb;
use super::firmware::{Firmware, Why};
use super::geometry::{decode, layout_of};
use super::record::{boot_fb, Listed};

pub fn bring_up<W: Firmware>(fw: &mut W, r: &Listed) -> Result<Fb, Why> {
    let size = boot_fb(r).ok_or(Why::Record)?;
    fw.announce()?;
    let epoch = fw.claim(r.device_id)?;
    let up = held(fw, r, epoch, size);
    if up.is_err() {
        fw.release(r.device_id);
    }
    up
}

fn held<W: Firmware>(fw: &mut W, r: &Listed, epoch: u64, size: u64) -> Result<Fb, Why> {
    let g = decode(&fw.geometry(r.device_id)?).ok_or(Why::Geometry)?;
    let Some(layout) = layout_of(g.format) else {
        fw.say("driver.bootfb: three-byte pixels are not served; the console keeps the screen");
        return Err(Why::Format);
    };
    // The mode's first pixel must be where the record put it.
    if g.offset != r.bar0.4 {
        return Err(Why::Shape);
    }
    let base = fw.map(r.device_id, epoch, size)?;
    let len = usize::try_from(size - g.offset as u64).map_err(|_| Why::Shape)?;
    let first = base.wrapping_add(g.offset as usize);
    let mut fb = Fb::new(first, len, (g.width, g.height), g.stride, layout).ok_or(Why::Shape)?;
    fb.physical_mm = g.physical_mm;
    Ok(fb)
}
