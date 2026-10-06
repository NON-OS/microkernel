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

//! The lines a photo of `log rtsx` should show: the reader up, and for each
//! card its size and whether its first block read, or where it stopped.

use crate::card::{read_blocks, Card};
use crate::chip::Family;
use crate::error::Result;
use crate::log::{emit, failed, Line};
use crate::setup::Driver;

pub fn up(drv: &Driver) {
    let name: &[u8] = match drv.family {
        Family::Rts5227 => b"RTS5227",
        Family::Rts522a => b"RTS522A",
    };
    let mut line = Line::start();
    line.text(name).text(b" (10ec:").hex(drv.device as u64, 4).text(b") up, IC version ");
    line.dec(drv.ic_version as u64);
    let slot: &[u8] = if crate::card::present(drv) { b"; card in slot" } else { b"; slot empty" };
    emit(line.text(slot));
}

pub fn card(drv: &Driver, card: Result<Card>) {
    let card = match card {
        Ok(c) => c,
        Err(e) => return failed(b"card bring-up stopped", e),
    };
    let mut line = Line::start();
    line.text(b"SD card ").dec(card.blocks).text(b" blocks (").dec(card.blocks / 2048);
    line.text(if card.high_capacity {
        b" MiB, block addressed)"
    } else {
        b" MiB, byte addressed)"
    });
    emit(line.text(b", rca ").hex(card.rca as u64, 4));
    if let Err(e) = read_blocks(drv, &card, 0, 1) {
        return failed(b"block 0 read", e);
    }
    let (a, b) = (drv.data.read_u8(510), drv.data.read_u8(511));
    let mut line = Line::start();
    line.text(b"block 0 read, last two bytes ").hex(a as u64, 2).hex(b as u64, 2);
    let mbr: &[u8] = if (a, b) == (0x55, 0xAA) { b" (partition table)" } else { b"" };
    emit(line.text(mbr));
}

pub fn removed(off: Result<()>) {
    match off {
        Ok(()) => emit(Line::start().text(b"card removed, slot powered down")),
        Err(e) => failed(b"card removed; slot power-down", e),
    }
}
