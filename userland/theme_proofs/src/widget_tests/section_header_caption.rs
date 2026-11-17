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

//! SectionHeader: a caption is one body-sized line in `muted`, exactly the
//! title as `text::left` draws it at the body size, a body line tall and
//! shorter than the title it replaces.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, SectionHeader};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::samples::LONG;
use super::serial::{serial, SCALES};

#[test]
fn section_header_caption_is_a_muted_body_line() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let px = scale().font(TextRole::Body.px());
        let lh = line_height(px) as u32;
        for title in ["Tab Groups", "Grüße — 日本語", LONG] {
            let c = SectionHeader::new(title).caption();
            let taller = SectionHeader::new(title).size().1;
            if c.size() != (text::width(title, px), lh) || lh >= taller {
                fails.push(format!("{title} at {q}/4: {:?}", c.size()));
            }
            for at in boxes(c.size()) {
                let top = (at.y + (at.h - lh) / 2) as i32;
                let line = text::truncate(title, at.w, px);
                let want = Canvas::new(at, 0)
                    .painted(|fb| _ = text::left(fb, at.x, top, &line, roles().muted, px));
                if Canvas::new(at, 0).painted(|fb| c.draw(fb, at)).px != want.px {
                    fails.push(format!("{title} in {at:?} at {q}/4 is not the muted body line"));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
