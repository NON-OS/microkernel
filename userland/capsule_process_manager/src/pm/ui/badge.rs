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

//! The shield a protected process wears after its name, in the process
//! tables and the authority matrix. The shared table draws text alone, so
//! the name is cut short enough to leave the shield room, and the shield is
//! drawn over the cell once the table is down.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::icons::{draw, IconId};
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{truncate, width};

use super::table_geom::row_h;
use super::text::valid;

const ROW_ICON: u32 = 13;
const ROW_ICON_GAP: u32 = 6;

/// The size the shared table draws its cells at.
fn px() -> f32 {
    scale().font(TextRole::Body.px())
}

/// The name as the table draws it in `room` pixels, cut when `protected` to
/// leave the shield and its gap.
pub fn name(bytes: &[u8], room: u32, protected: bool) -> String {
    let s = scale();
    let mark = if protected { s.px(ROW_ICON) + s.px(ROW_ICON_GAP) } else { 0 };
    truncate(valid(bytes), room.saturating_sub(mark), px())
}

/// The shield after `cut`, a name drawn from `x` in the row whose top is `y`.
pub fn shield(fb: &mut PaintBuffer, x: u32, y: u32, cut: &str) {
    let (s, accent) = (scale(), roles().accent.argb());
    let (icon, at) = (s.px(ROW_ICON), x + width(cut, px()) + s.px(ROW_ICON_GAP));
    let top = y + row_h().saturating_sub(icon) / 2;
    draw(fb, IconId::SettingsSecurity, at, top, icon, accent);
}
