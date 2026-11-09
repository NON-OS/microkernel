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

use nonos_app_skeleton::PaintBuffer;
use nonos_libc::mk_time_millis;
use nonos_toolkit::widgets::Controls;

use crate::browser::paint::pages::{body, extent};
use crate::browser::paint::tokens::palette;
use crate::browser::shell::Shell;
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_part::PagePart;

/// A list page, into a buffer the size of the content area, its controls
/// laid out afresh in `c`.
pub fn paint(shell: &Shell, fb: &mut PaintBuffer, c: &mut Controls<PagePart>) {
    fb.fill_rect(0, 0, fb.width, fb.height, palette().ground);
    let (w, h) = (fb.width as i32, fb.height as i32);
    let g = ListGeom::new(w, h, extent::page_scroll(shell, w, h));
    body::draw(shell, &g, mk_time_millis(), fb, c);
}
