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
//! the serial, and what it holds now. A driver that did not answer gets
//! the same row in the fault colour.

use nonos_app_skeleton::PaintBuffer;
use nonos_blk_client::{Contents, Disk};

use crate::install::format::bytes;
use crate::install::ui::metrics::{BODY_PX, SMALL_PX};
use crate::install::ui::text::right;
use crate::install::ui::{text, theme};

pub fn row(fb: &mut PaintBuffer, d: &Disk, bx: u32, y: u32, bw: u32) {
    let x = bx + 18;
    let top = y + 8;
    match &d.fault {
        None => {
            let name = d.identity.map(|i| alloc::string::String::from(i.model_str()));
            let title = name.as_deref().unwrap_or(d.label());
            text::line(fb, x, top, title, theme::TITLE, BODY_PX);
            right(fb, bx + bw - 18, top, &bytes(d.bytes()), theme::FOREGROUND, BODY_PX);
            let colour = match d.contents {
                Contents::Blank => theme::MUTED,
                Contents::Nonos => theme::ACCENT,
                _ => theme::WARN,
            };
            let sub = match d.identity {
                Some(i) => alloc::format!(
                    "{}  serial {}  {}",
                    d.label(),
                    i.serial_str(),
                    d.contents.text()
                ),
                None => alloc::format!("{}  {}", d.label(), d.contents.text()),
            };
            text::line(fb, x, top + 24, &sub, colour, SMALL_PX);
        }
        Some(fault) => {
            text::line(fb, x, top, d.label(), theme::MUTED, BODY_PX);
            text::line(fb, x, top + 24, fault, theme::DANGER, SMALL_PX);
        }
    }
}
