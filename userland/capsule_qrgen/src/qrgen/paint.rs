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
use nonos_toolkit::theme::roles;

use super::code::{Matrix, PAYLOAD};
use super::draw::{draw_matrix, margin};

const TITLE_Y: u32 = 20;
const PAYLOAD_Y: u32 = 48;
const MATRIX_TOP: u32 = 76;
/// The accent band's height and how far below the code the hint sits, in
/// logical pixels, as the title, payload and code tops above are.
const BAND: u32 = 4;
const HINT_GAP: u32 = 12;
/// The title's, the payload's, the error's and the hint's font sizes, in
/// logical pixels.
const TITLE_PX: f32 = 19.0;
const PAYLOAD_PX: f32 = 14.0;
const ERROR_PX: f32 = 15.0;
const HINT_PX: f32 = 13.0;

pub fn paint(fb: &mut PaintBuffer, matrix: Option<&Matrix>) {
    let (r, s, left) = (roles(), scale(), margin());
    let (title_y, payload_y) = (s.px(TITLE_Y) as i32, s.px(PAYLOAD_Y) as i32);
    let matrix_top = s.px(MATRIX_TOP);
    fb.clear(r.bg.argb());
    fb.fill_rect(0, 0, fb.width, s.px(BAND), r.accent.argb());
    fb.text_ttf(left, title_y, "qr generator", r.accent.argb(), s.font(TITLE_PX));
    fb.text_ttf(left, payload_y, PAYLOAD, r.text.argb(), s.font(PAYLOAD_PX));
    let Some(matrix) = matrix else {
        let ink = r.danger.argb();
        fb.text_ttf(left, matrix_top as i32, "encode failed", ink, s.font(ERROR_PX));
        return;
    };
    let hint_y = (draw_matrix(fb, matrix, matrix_top) + s.px(HINT_GAP)) as i32;
    fb.text_ttf(left, hint_y, "esc closes this window", r.quiet.argb(), s.font(HINT_PX));
}
