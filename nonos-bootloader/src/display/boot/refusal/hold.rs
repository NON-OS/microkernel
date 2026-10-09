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

//! Keep a refusal on screen long enough to read: until a key is pressed, or
//! 30 s pass. The refusal is already final; this only delays the restart that
//! follows, and never waits without boot services.

use uefi::proto::console::text::Input;
use uefi::table::boot::BootServices;

use crate::display::fx::clear_region;
use crate::display::gop::{get_dimensions, hline};
use crate::display::ink::palette::{BAD, BORDER, TEXT_2, TEXT_3};
use crate::display::ink::{label, label_width, metrics, round_rect, Scene, Style};
use crate::display::text::Text;
use crate::log::get_boot_services;

const HOLD_S: u32 = 30;
const TICK_US: usize = 100_000;
const KEYS: &[u8] = b"ANY KEY RESTARTS NOW";

pub fn hold(s: &Scene) {
    let ptr = get_boot_services();
    if ptr.is_null() {
        return;
    }
    /* The logger keeps the firmware's boot services table, valid until exit. */
    let bs: &BootServices = unsafe { &*ptr };
    let (w, h) = get_dimensions();
    let (u, mono) = (s.u, metrics(Style::Mono));
    let (side, y) = (8 * u, s.footer_y + mono.line + 2 * u);
    for tick in 0..HOLD_S * 10 {
        if tick % 10 == 0 {
            let left = HOLD_S - tick / 10;
            clear_region(0, s.footer_y, w, h.saturating_sub(s.footer_y));
            hline(0, s.footer_y, w, BORDER);
            let t = Text::new().push(b"RESTART IN ").dec(left as u64).push(b" S");
            let tw = label(side, y, t.as_bytes(), TEXT_2);
            round_rect(side, y + mono.line + u, (tw * left / HOLD_S).max(2), 2, 1, BAD);
            label(w.saturating_sub(side + label_width(KEYS)), y, KEYS, TEXT_3);
        }
        if key_pressed(bs) {
            return;
        }
        bs.stall(TICK_US);
    }
}

fn key_pressed(bs: &BootServices) -> bool {
    let Ok(handle) = bs.get_handle_for_protocol::<Input>() else {
        return false;
    };
    let Ok(mut input) = bs.open_protocol_exclusive::<Input>(handle) else {
        return false;
    };
    matches!(input.read_key(), Ok(Some(_)))
}
