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

//! The bring-up: the claim and the port grant, SVGA version 2, the FIFO
//! mapped and laid out, the mode chosen and set, the framebuffer BAR
//! mapped from its start for as much as VRAM holds, so a later mode set
//! finds its frame mapped. A failing step drops the handles, which put the
//! firmware's mode back and give back every grant and the claim.

use nonos_gfx_serve::{Platform, Server};

use super::{adapter, claim, map::map, restore::Saved, Driver};
use crate::constants::regs::{CAPABILITIES, MAX_HEIGHT, MAX_WIDTH, MEM_SIZE, VRAM_SIZE};
use crate::discover::Found;
use crate::fifo::{configure, Fifo};
use crate::handles::BrokerHandles;
use crate::init::{firmware_mode, negotiate, set_mode};
use crate::regs::{Port, Regs};
use crate::screen::{choose, Limits};

pub fn run<P: Platform>(dev: Found, platform: P) -> Result<Driver<P>, &'static str> {
    let (epoch, pio) = claim::claim(dev.device_id, dev.layout.port)?;
    let mut handles = BrokerHandles::new(dev.device_id, pio);
    let regs = Regs::new(Port::new(pio));
    negotiate(&regs)?;
    handles.restore_at_drop(regs, Saved::read(&regs));
    let fifo_len = (regs.read(MEM_SIZE) as u64).min(dev.fifo_bytes) & !0xFFF;
    let fifo_map = map(dev.device_id, epoch, dev.layout.fifo, 0, fifo_len)?;
    handles.mapped(fifo_map.grant_id);
    let mut fifo = Fifo::new(fifo_map.user_va, fifo_len);
    configure(&regs, &mut fifo, regs.read(CAPABILITIES))?;
    let vram = regs.read(VRAM_SIZE);
    let max = (regs.read(MAX_WIDTH), regs.read(MAX_HEIGHT));
    let (w, h) = choose(firmware_mode(&regs), Limits { max, vram }).ok_or("no mode fits VRAM")?;
    let layout = set_mode(&regs, (w, h))?;
    let fb_len = (vram as u64).min(dev.fb_bytes) & !0xFFF;
    let fb_map = map(dev.device_id, epoch, dev.layout.fb, 0, fb_len)?;
    handles.mapped(fb_map.grant_id);
    let window = (fb_map.user_va, fb_len);
    let svga = adapter::adapter((regs, fifo), window, (max, vram), (w, h), layout)?;
    Ok(Driver { handles, server: Server::new(svga, platform) })
}
