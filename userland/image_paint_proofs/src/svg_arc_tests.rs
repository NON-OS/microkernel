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

//! The hero arc of nonos.software rasterized at 400 x 400. Its stops say
//! stop-color="var(--arc-a, var(--accent))": a presentation attribute is
//! parsed against the property's grammar, not as a declaration (SVG 2,
//! Presentation attributes and Attribute syntax), var() does not match
//! <color>, and an attribute that fails to parse is taken as the initial
//! value, black for stop-color (SVG 2, Paint Servers). Both stops are
//! black, so the stroke is opaque black.
use crate::svg_arc::{arc, px, raster, OFF, ON};

const VAR: [&str; 2] = ["var(--arc-a, var(--accent))", "var(--arc-b, var(--accent))"];

#[test]
fn the_hero_arc_strokes_with_its_gradient_of_initial_stops() {
    let (w, h, svg, p) = raster(&arc(VAR, "url(#arc-ink)"));
    assert_eq!((w, h), (400, 400));
    for name in ["<linearGradient ", "viewBox=", "preserveAspectRatio="] {
        assert!(svg.contains(name), "{name} keeps its SVG case");
    }
    assert_eq!(px(&p, w, ON.0, ON.1), 0xFF00_0000, "var() stops take initial black");
    assert_eq!(px(&p, w, OFF.0, OFF.1), 0, "nothing off the path");
}

#[test]
fn url_arc_ink_resolves_to_the_gradient() {
    let (w, _, _, p) = raster(&arc(["#3a52ff", "#66ffff"], "url(#arc-ink)"));
    let c = px(&p, w, ON.0, ON.1);
    let (r, g, b) = (c >> 16 & 0xff, c >> 8 & 0xff, c & 0xff);
    assert_eq!(c >> 24, 0xff);
    assert!((0x3a..=0x66).contains(&r) && (0x52..=0xff).contains(&g) && b == 0xff, "{c:08x}");
}

#[test]
fn a_paint_server_that_does_not_resolve_paints_none_or_its_fallback() {
    let (w, _, _, p) = raster(&arc(VAR, "url(#missing)"));
    assert!(p.iter().all(|&c| c == 0), "no paint, never black");
    let (w2, _, _, p2) = raster(&arc(VAR, "url(#missing) #ff0000"));
    assert_eq!(px(&p2, w2, ON.0, ON.1), 0xFFFF_0000, "the fallback colour");
    assert_eq!(w, w2);
}

#[test]
fn an_svg_without_a_viewbox_maps_one_unit_to_one_css_px() {
    let rect = "<rect width=\"50\" height=\"50\" fill=\"#ff0000\"/>";
    for attrs in ["", " width=\"100\" height=\"50\""] {
        let body = std::format!("<svg{attrs} style=\"width:200px;height:100px\">{rect}</svg>");
        let (w, h, svg, p) = raster(&body);
        assert_eq!((w, h), (200, 100));
        assert!(svg.contains("width=\"200\" height=\"100\""), "{svg}");
        assert_eq!(px(&p, w, 45, 45), 0xFFFF_0000, "{attrs}: the rect reaches 50 px");
        assert_eq!(px(&p, w, 55, 10), 0, "{attrs}: and stops there across");
        assert_eq!(px(&p, w, 10, 55), 0, "{attrs}: and down");
    }
}
