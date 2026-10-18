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

//! SVGA_CMD_UPDATE: the host redraws a rectangle of the framebuffer. A full
//! ring is waited on, pinging the host with SYNC, for a bounded time; past
//! it the update is dropped and the next frame redraws the area.

use nonos_libc::Deadline;

use super::ring::Fifo;
use crate::constants::fifo::{CMD_UPDATE, SPACE_WAIT_MS};
use crate::constants::regs::{SYNC, SYNC_GENERIC};
use crate::regs::Regs;

pub fn update(
    regs: &Regs,
    fifo: &Fifo,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<(), &'static str> {
    let words = [CMD_UPDATE, x, y, w, h];
    let need = (words.len() * 4) as u32;
    let deadline = Deadline::after_ms(SPACE_WAIT_MS);
    loop {
        match fifo.free() {
            None => return Err("FIFO indices out of range"),
            Some(free) if free >= need => break,
            Some(_) if deadline.expired() => return Err("FIFO stayed full"),
            Some(_) => regs.write(SYNC, SYNC_GENERIC),
        }
        core::hint::spin_loop();
    }
    fifo.push(&words);
    regs.write(SYNC, SYNC_GENERIC);
    Ok(())
}
