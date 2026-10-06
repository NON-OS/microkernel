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

#![cfg(test)]
/* CSS filter: the color functions compose into one map applied to what a
 * filtered box paints, including an inline image, and the functions this
 * renderer does not draw are passed over. */

use crate::browser::layout::boxmodel::Content;
use crate::browser::layout::filter_table::{tint_of, tint_px};
use crate::render::render_with;

fn map(v: &str) -> Option<u32> {
    let html = format!("<div id=d style=\"height:9px;filter:{v}\"></div>");
    let doc = render_with(&html, (400, 300), &|_| None);
    let t = doc.frags.iter().map(|f| f.tint).find(|&t| t != 0)?;
    Some(tint_px(&tint_of(t)?, 0xffff_0000))
}

#[test]
fn color_functions_map_a_pixel() {
    assert_eq!(map("grayscale(1)"), Some(0xff36_3636), "0.2126 of red");
    assert_eq!(map("brightness(0) invert(1)"), Some(0xffff_ffff), "any color to white");
    assert_eq!(map("brightness(50%)"), Some(0xff80_0000));
    assert_eq!(map("invert(100%)"), Some(0xff00_ffff));
    assert_eq!(map("opacity(.5)").map(|p| p >> 24), Some(0x80));
    assert_eq!(map("hue-rotate(0deg) saturate(1)"), Some(0xffff_0000), "identity");
}

#[test]
fn functions_this_renderer_does_not_draw_are_passed_over() {
    assert_eq!(map("blur(4px)"), None, "a lone blur leaves the box unfiltered");
    assert_eq!(map("none"), None);
    assert_eq!(map("blur(4px) grayscale(1)"), Some(0xff36_3636));
}

#[test]
fn an_inline_image_carries_its_filter() {
    let html = "<p><img id=a src=a.png style=\"filter:grayscale(1)\"><img id=b src=b.png></p>";
    let doc = render_with(html, (400, 300), &|_| Some((20, 10)));
    let tints: Vec<u16> = doc
        .frags
        .iter()
        .filter(|f| matches!(f.content, Content::Image { .. }))
        .map(|f| f.tint)
        .collect();
    assert_eq!(tints.len(), 2);
    assert!(tints[0] != 0 && tints[1] == 0, "{tints:?}");
}
