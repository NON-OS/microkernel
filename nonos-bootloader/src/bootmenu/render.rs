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

use super::entries::ENTRIES;
use super::footer::draw_footer;
use super::header::draw_header;
use super::list::draw_list;
use super::security_status::draw_security_status;
use crate::display::fx::fill_atmosphere;
use crate::display::gop::get_dimensions;
use crate::security::SecurityContext;

pub(super) fn render(sel: usize, default: usize, remaining_s: u32, sec: &SecurityContext) {
    let (w, h) = get_dimensions();
    fill_atmosphere();

    // Vertically center the title + list + status as one block.
    let rows = ENTRIES.len() as u32 * 50;
    let title_y = h.saturating_sub(60 + rows) / 2 + 24;
    let list_top = title_y + 116;
    let status_y = list_top + rows + 30;

    draw_header(w, title_y);
    draw_list(w, list_top, sel);
    draw_footer(w, status_y, remaining_s, default_name(default));
    draw_security_status(w, status_y + 44, sec);
}

fn default_name(default: usize) -> &'static [u8] {
    if default == 0 {
        b"Hardened"
    } else {
        b"Standard"
    }
}
