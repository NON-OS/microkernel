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

//! The left navigation rail: the brand over the nav list. The account block
//! that sat at its foot named a person ("Mehedi Hasan, Local Account") the
//! machine knows nothing of; the editor has no accounts, so it is gone.

use nonos_app_skeleton::PaintBuffer;

use crate::editor::widget::{paint_navlist, NavStyle};

use super::brand::paint_brand;
use super::metrics::{nav_rect, rail_x, BODY, RAIL_W};
use super::palette::{LABEL, NAV_ACCENT, NAV_RING, RAIL_BG, RAIL_LINE, TITLE};
use super::state::{HomeState, NAV_LABELS};

pub(super) fn paint_rail(fb: &mut PaintBuffer, st: &HomeState) {
    let h = fb.height;
    fb.fill_rect(rail_x(), 0, RAIL_W - 1, h, RAIL_BG);
    fb.fill_rect(rail_x() + RAIL_W - 1, 0, 1, h, RAIL_LINE);
    paint_brand(fb);
    let style = NavStyle {
        accent: NAV_ACCENT,
        ring: NAV_RING,
        label: LABEL,
        label_sel: TITLE,
        radius: 9,
        pad_x: 14,
    };
    paint_navlist(fb, nav_rect(), &NAV_LABELS, st.nav, BODY, &style);
}
