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

//! One disk on the list: what the part calls itself, its size, the bus,
//! the serial, and what it holds now. A row that is not a target says why
//! in place of what it holds: in the fault colour for a driver that is
//! missing or did not answer, in the warning colour for a disk that may
//! be the one this boot came from.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_blk_client::{Contents, Disk};

use crate::install::format::bytes;
use crate::install::ui::metrics::Metrics;
use crate::install::ui::text::right;
use crate::install::ui::{text, theme};

pub fn row(fb: &mut PaintBuffer, m: &Metrics, d: &Disk, bx: u32, y: u32, bw: u32) {
    let x = bx + m.inset;
    let top = y + m.unit;
    let second = top + m.line_h;
    let name = d.identity.map(|i| String::from(i.model_str())).filter(|s| !s.is_empty());
    let title = name.as_deref().unwrap_or(d.label());
    let lit = if d.device.is_some() { theme::TITLE } else { theme::MUTED };
    text::line(fb, x, top, title, lit, m.body_px);
    if d.bytes() > 0 {
        right(fb, bx + bw - m.inset, top, &bytes(d.bytes()), theme::FOREGROUND, m.body_px);
    }
    let (sub, colour) = match &d.fault {
        None => (described(d), held_colour(d.contents)),
        Some(why) if d.bytes() > 0 => (alloc::format!("{}  {why}", d.label()), theme::WARN),
        Some(why) => (String::from(why.as_str()), theme::DANGER),
    };
    text::line(fb, x, second, &sub, colour, m.small_px);
}

/* The bus, the serial when the part has one, a block size that is not the
 * usual 512 bytes, and what the disk holds now. */
fn described(d: &Disk) -> String {
    let mut s = String::from(d.label());
    if let Some(serial) = d.identity.as_ref().map(|i| i.serial_str()).filter(|s| !s.is_empty()) {
        s.push_str("  serial ");
        s.push_str(serial);
    }
    if d.block > 512 {
        s.push_str(&alloc::format!("  {}-byte blocks", d.block));
    }
    s.push_str("  ");
    s.push_str(d.contents.text());
    s
}

fn held_colour(c: Contents) -> u32 {
    match c {
        Contents::Blank => theme::MUTED,
        Contents::Nonos => theme::ACCENT,
        _ => theme::WARN,
    }
}
