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

use alloc::format;
use alloc::string::String;

use crate::browser::css::{Computed, Size};

use super::attr_px::attr_px;
use super::leaf::leaf;
use super::svg_serialize::serialize_svg;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

/* Box for an inline <svg>, its subtree serialized into a data URL for the image
 * rasterizer. Width/height attributes stand in for auto CSS sizes; one definite
 * side sets the other by the viewBox aspect (else 150 or 300), none is 300x150. */
pub(super) fn element_svg(
    w: &Walk,
    item: &ElementIn,
    parent: &Computed,
    link: &Option<String>,
    style: Computed,
) -> BoxNode {
    let svg = serialize_svg(w.dom, item.ch);
    let src = format!("data:image/svg+xml,{svg}");
    let mut b = leaf(BoxKind::Image { src, alt: String::new() }, parent, link, item.ch);
    b.style = style;
    for (side, name) in [(&mut b.style.width, "width"), (&mut b.style.height, "height")] {
        if *side == Size::Auto {
            if let Some(px) = attr_px(item.c.attr(name)) {
                *side = Size::Px(px);
            }
        }
    }
    let ratio = view_box(item.c.attr("viewBox"));
    let (bw, bh) = (b.style.width.definite_px(), b.style.height.definite_px());
    let auto = (b.style.width == Size::Auto, b.style.height == Size::Auto);
    let scaled = |v: i32, num: f32, den: f32| Size::Px((v.max(0) as f32 * num / den + 0.5) as u32);
    match (auto, bw, bh, ratio) {
        ((true, true), _, _, None) => {
            (b.style.width, b.style.height) = (Size::Px(300), Size::Px(150))
        }
        ((_, true), Some(w), _, Some((vw, vh))) => b.style.height = scaled(w, vh, vw),
        ((true, _), _, Some(h), Some((vw, vh))) => b.style.width = scaled(h, vw, vh),
        ((_, true), Some(_), _, None) => b.style.height = Size::Px(150),
        ((true, _), _, Some(_), None) => b.style.width = Size::Px(300),
        _ => {}
    }
    b
}

/* The viewBox width and height when both are positive numbers. */
fn view_box(v: Option<&str>) -> Option<(f32, f32)> {
    let mut it = v?.split(|c: char| c == ',' || c.is_ascii_whitespace()).filter(|p| !p.is_empty());
    let mut nums = [0f32; 4];
    for n in nums.iter_mut() {
        *n = it.next()?.parse::<f32>().ok()?;
    }
    (nums[2] > 0.0 && nums[3] > 0.0 && nums[2].is_finite() && nums[3].is_finite())
        .then_some((nums[2], nums[3]))
}
