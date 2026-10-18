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

//! The bring-up: claim and the port grant, SVGA version 2, the FIFO mapped
//! and laid out, the mode chosen and set, the visible framebuffer mapped,
//! the surface made. A failing step puts the firmware's mode back and gives
//! back every grant and the claim.

use nonos_libc::{mk_device_release, mk_mmio_unmap, mk_pio_release};

use super::{claim, driver::Driver, map::map, restore::Saved, surface};
use crate::constants::regs::{CAPABILITIES, MAX_HEIGHT, MAX_WIDTH, MEM_SIZE, VRAM_SIZE};
use crate::discover::Found;
use crate::fifo::{configure, Fifo};
use crate::init::{firmware_mode, negotiate, set_mode};
use crate::regs::{Port, Regs};
use crate::screen::{choose, Limits, Plane};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let (epoch, pio) = claim::claim(dev.device_id, dev.layout.port)?;
    let regs = Regs::new(Port::new(pio));
    let back = |maps: &[u64], e: &'static str| {
        for &g in maps.iter().rev() {
            let _ = mk_mmio_unmap(g);
        }
        let _ = mk_pio_release(pio);
        let _ = mk_device_release(dev.device_id);
        e
    };
    negotiate(&regs).map_err(|e| back(&[], e))?;
    let saved = Saved::read(&regs);
    let fifo_len = (regs.read(MEM_SIZE) as u64).min(dev.fifo_bytes) & !0xFFF;
    let fifo_map =
        map(dev.device_id, epoch, dev.layout.fifo, 0, fifo_len).map_err(|e| back(&[], e))?;
    let undo = |maps: &[u64], e| {
        saved.restore(&regs);
        back(maps, e)
    };
    let mut fifo = Fifo::new(fifo_map.user_va, fifo_len);
    configure(&regs, &mut fifo, regs.read(CAPABILITIES))
        .map_err(|e| undo(&[fifo_map.grant_id], e))?;
    let lim =
        Limits { max: (regs.read(MAX_WIDTH), regs.read(MAX_HEIGHT)), vram: regs.read(VRAM_SIZE) };
    let (w, h) = choose(firmware_mode(&regs), lim)
        .ok_or("no mode fits VRAM")
        .map_err(|e| undo(&[fifo_map.grant_id], e))?;
    let layout = set_mode(&regs, (w, h)).map_err(|e| undo(&[fifo_map.grant_id], e))?;
    let fb_len = layout.pitch as u64 * h as u64;
    let end = layout.offset as u64 + fb_len;
    if end > lim.vram as u64 || end > dev.fb_bytes {
        return Err(undo(&[fifo_map.grant_id], "framebuffer past VRAM"));
    }
    let fb_map = map(dev.device_id, epoch, dev.layout.fb, layout.offset as u64, fb_len)
        .map_err(|e| undo(&[fifo_map.grant_id], e))?;
    let (surface, sid) =
        surface::create(w, h).map_err(|e| undo(&[fifo_map.grant_id, fb_map.grant_id], e))?;
    let fb = Plane { va: fb_map.user_va, stride: layout.pitch, width: w, height: h };
    Ok(Driver { regs, fifo, fb, surface, sid, handle: 0, owner: 0 })
}
