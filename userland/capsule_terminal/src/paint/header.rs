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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Rect, Row, Separator};

/// The band's height in logical pixels.
const HEADER_H: u32 = 28;
const MARK: &str = "\u{00D8} NONOS";

/// The band above the body, scaled with the interface.
pub fn header_h() -> u32 {
    scale().px(HEADER_H)
}

/// The working directory as the header shows it: its end, behind an ellipsis
/// once it runs past the half of the row a `Row` gives its value.
pub fn cwd_shown(cwd: &str, w: u32) -> alloc::string::String {
    let lane = w.saturating_sub(scale().px(u32::from(SPACE.lg)).saturating_mul(2));
    text::tail(cwd, lane / 2, scale().font(TextRole::Body.px()))
}

/// The system's mark and the working directory as one row on the window's
/// chrome, with a hairline under it.
pub fn draw_header(cwd: &str, fb: &mut PaintBuffer) {
    let band = Rect { x: 0, y: 0, w: fb.width, h: header_h() };
    draw::fill(fb, band, roles().surface);
    Row::new(MARK).value(&cwd_shown(cwd, band.w)).draw(fb, band);
    Separator::new().draw(fb, Rect { y: band.h, h: scale().hair(), ..band });
}
