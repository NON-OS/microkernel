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

use super::about::draw_about;
use super::brand::{draw_captions, draw_emblem};
use super::footer::draw_footer;
use super::layout::{layout, Layout};
use super::list::draw_list;
use super::platform::draw_platform;
use super::state::{Dirty, Frame};
use crate::display::fx::{clear_region, fill_atmosphere};
use crate::display::gop::get_dimensions;

pub(super) fn render(f: &Frame<'_>, dirty: Dirty) {
    let l = layout();
    match dirty {
        Dirty::All => {
            fill_atmosphere();
            draw_captions(&l);
            draw_emblem(&l, if f.intro { 0 } else { 1000 });
            if f.intro {
                return;
            }
            draw_platform(&l, f.sec);
            draw_body(&l, f);
        }
        Dirty::Selection => {
            let top = l.list_y.saturating_sub(l.row_h);
            clear_region(l.col_x, top, l.col_w, l.platform_y.saturating_sub(top));
            draw_body(&l, f);
        }
        Dirty::Timer => {}
    }
    let (w, h) = get_dimensions();
    clear_region(0, l.footer_y, w, h.saturating_sub(l.footer_y));
    draw_footer(&l, f.remaining_s, f.total_s, f.default);
}

fn draw_body(l: &Layout, f: &Frame<'_>) {
    draw_list(l, f.sel, f.default);
    draw_about(l, f.sel, f.sec);
}
