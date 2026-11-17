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

//! A page carried into device px draws its text at the scale, a press where a
//! link is drawn finds that link, and the resolution media features see the
//! scale. A binary of its own, so no other test moves the scale latch.

use capsule_browser_proofs::browser::css::color::media_query_matches;
use capsule_browser_proofs::browser::layout::boxmodel::{css_px, doc_to_device, Content};
use capsule_browser_proofs::render::render_full;
use nonos_toolkit::design::scale::set_scale;

const PAGE: &str = "<body style='margin:0'>\
    <div id=b style='position:absolute;left:10px;top:21px;width:101px;height:33px;\
    background:#f00'></div><div id=w style='height:7px;background:#0f0'></div>\
    <a href='/x' style='position:absolute;left:30px;top:203px;font-size:15px'>link</a>";

const PANE: (u32, u32) = (1301, 703);

fn text_px(doc: &capsule_browser_proofs::browser::layout::boxmodel::BoxDocument) -> f32 {
    let px = doc.frags.iter().find_map(|f| match f.content {
        Content::Text { px, .. } => Some(px),
        _ => None,
    });
    px.expect("no text laid out")
}

fn at(q: u32) {
    set_scale(q);
    let vp = (css_px(PANE.0 as i32) as u32, css_px(PANE.1 as i32) as u32);
    let (_, css) = render_full(PAGE, vp, &|_| None);
    let (_, mut dev) = render_full(PAGE, vp, &|_| None);
    doc_to_device(&mut dev, q);
    assert_eq!(text_px(&dev), text_px(&css) * q as f32 / 4.0, "q={q}: text px");
    let link = dev.frags.iter().find(|f| f.href.is_some()).unwrap();
    let (cx, cy) = (link.x + link.w / 2, link.y + link.h / 2);
    assert_eq!(dev.link_at(cx, cy, 0), Some("/x"), "q={q}: a press on the drawn link");
    assert!(dev.content_h >= css.content_h, "q={q}: the page is as tall on the device");
    let dppx = format!("(min-resolution: {}dppx)", q as f32 / 4.0);
    assert!(media_query_matches(&dppx, vp.0, vp.1), "q={q}: {dppx}");
    let above = format!("(min-resolution: {}dppx)", q as f32 / 4.0 + 0.25);
    assert!(!media_query_matches(&above, vp.0, vp.1), "q={q}: {above}");
}

#[test]
fn page_text_links_and_resolution_follow_the_display_scale() {
    for q in [4, 5, 6, 8] {
        at(q);
    }
    set_scale(4);
}
