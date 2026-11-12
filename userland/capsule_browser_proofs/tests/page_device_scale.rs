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

//! A page laid out in CSS px over the display scale and carried into device
//! px grows its boxes by the scale; 1x is the layout unchanged.

use capsule_browser_proofs::browser::layout::boxmodel::{css_px, doc_to_device};
use capsule_browser_proofs::browser::layout::boxmodel::{BoxDocument, Content, Fragment};
use capsule_browser_proofs::render::render_full;
use nonos_toolkit::design::scale::{scale, set_scale};

const PAGE: &str = "<body style='margin:0'>\
    <div id=b style='position:absolute;left:10px;top:21px;width:101px;height:33px;\
    background:#f00'></div><div id=w style='height:7px;background:#0f0'></div>\
    <a href='/x' style='position:absolute;left:30px;top:203px;font-size:15px'>link</a>";

const PANE: (u32, u32) = (1301, 703);

fn print(doc: &BoxDocument) -> Vec<(i32, i32, i32, i32, u32, Option<u32>)> {
    let px = |f: &Fragment| match f.content {
        Content::Text { px, .. } => Some(px.to_bits()),
        _ => None,
    };
    doc.frags.iter().map(|f| (f.x, f.y, f.w, f.h, f.bg, px(f))).collect()
}

fn near(css: i32, dev: i32, q: u32) -> bool {
    (css as i64 * q as i64 - dev as i64 * 4).abs() <= 2 + q as i64
}

fn at(q: u32, base: &[(i32, i32, i32, i32, u32, Option<u32>)]) {
    set_scale(q);
    let vp = (css_px(PANE.0 as i32) as u32, css_px(PANE.1 as i32) as u32);
    assert_eq!(vp, (PANE.0 * 4 / q, PANE.1 * 4 / q), "q={q}: the CSS viewport");
    let (dom, css) = render_full(PAGE, vp, &|_| None);
    let (_, mut dev) = render_full(PAGE, vp, &|_| None);
    doc_to_device(&mut dev, scale().quarters());
    let id = |name: &str| dom.nodes.iter().position(|n| n.attr("id") == Some(name)).unwrap();
    let wide = css.frags.iter().find(|f| f.node == id("w")).map(|f| f.w as u32);
    assert_eq!(wide, Some(vp.0), "q={q}: a block fills the CSS viewport");
    for (c, d) in css.frags.iter().zip(dev.frags.iter()) {
        let ok = [(c.x, d.x), (c.y, d.y), (c.w, d.w), (c.h, d.h)].iter().all(|p| near(p.0, p.1, q));
        assert!(ok, "q={q}: device box {:?} is not z x {:?}", (d.x, d.y, d.w, d.h), (c.x, c.y));
    }
    let b = dev.frags.iter().find(|f| f.node == id("b")).unwrap();
    let want = |v: i32| (v * q as i32 + 2) / 4;
    let edges = (want(10), want(21), want(111) - want(10), want(54) - want(21));
    assert_eq!((b.x, b.y, b.w, b.h), edges, "q={q}: the absolute div's edges");
    assert!(q != 4 || print(&dev) == base, "1x is the layout unchanged");
}

#[test]
fn pages_lay_out_in_css_px_and_draw_at_the_display_scale() {
    set_scale(4);
    let vp = (css_px(PANE.0 as i32) as u32, css_px(PANE.1 as i32) as u32);
    let base = print(&render_full(PAGE, vp, &|_| None).1);
    for q in [4, 5, 6, 8] {
        at(q, &base);
    }
    set_scale(4);
}
