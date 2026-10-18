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

//! The lines a photo of `log rtsx` should show: the reader up, each card
//! served with its size, and what happened in the slot since. A card is
//! named by its size and nothing else: its CID, serial included, stays off
//! the console (C5).

use crate::card::{present, write_protected, Card};
use crate::chip::Family;
use crate::log::{emit, Line};
use crate::protocol::SERVICE_NAME;
use crate::setup::Driver;

pub fn up(drv: &Driver) {
    let name: &[u8] = match drv.family {
        Family::Rts5227 => b"RTS5227",
        Family::Rts522a => b"RTS522A",
    };
    let mut line = Line::start();
    line.text(name).text(b" (10ec:").hex(drv.device as u64, 4).text(b") up, IC version ");
    line.dec(drv.ic_version as u64);
    let slot: &[u8] = if present(drv) { b"; card in slot" } else { b"; slot empty" };
    emit(line.text(slot));
}

pub fn served(drv: &Driver, card: &Card) {
    let mut line = Line::start();
    line.text(b"SD card ").dec(card.blocks).text(b" blocks (").dec(card.blocks / 2048);
    line.text(if card.high_capacity {
        b" MiB, block addressed)"
    } else {
        b" MiB, byte addressed)"
    });
    let wp: &[u8] = if write_protected(drv) == Ok(true) { b", write-protected" } else { b"" };
    emit(line.text(wp).text(b", served as ").text(SERVICE_NAME.as_bytes()));
}

pub fn said(what: &[u8]) {
    emit(Line::start().text(what));
}
