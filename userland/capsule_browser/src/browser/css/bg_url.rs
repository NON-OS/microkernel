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

use alloc::string::{String, ToString};

use super::calc::split_top::{items, words};

mod pos_parts;
mod shorthand;
mod size_parts;
mod top_slash;

pub(super) use pos_parts::apply_bg_pos;
pub(super) use shorthand::{apply_background, apply_bg_size};

/* The background layer captured from a background or background-image
 * declaration: the first comma layer (the topmost painted) that holds a
 * url() to fetch or a gradient kept verbatim for the painter. Solid
 * colours and unknown values give None. */
pub(super) fn bg_url(name: &str, value: &str) -> Option<String> {
    if name != "background" && name != "background-image" {
        return None;
    }
    let layer = image_layer(value)?;
    for w in words(layer) {
        if starts_ci(w, "url(") {
            let inner = w[4..].strip_suffix(')').unwrap_or(&w[4..]);
            let inner = inner.trim().trim_matches('"').trim_matches('\'').trim();
            return (!inner.is_empty() && !inner.starts_with("data:")).then(|| inner.to_string());
        }
        if is_gradient(w) {
            return Some(w.trim().to_string());
        }
    }
    None
}

/// The first comma layer at paren depth 0 that paints an image.
pub(super) fn image_layer(value: &str) -> Option<&str> {
    items(value).find(|l| words(l).any(|w| is_gradient(w) || starts_ci(w, "url(")))
}

/* The painter draws linear and radial gradients; repeating ones are not
 * captured rather than drawn as a single run. */
fn is_gradient(w: &str) -> bool {
    starts_ci(w, "linear-gradient(") || starts_ci(w, "radial-gradient(")
}

fn starts_ci(w: &str, p: &str) -> bool {
    w.get(..p.len()).is_some_and(|h| h.eq_ignore_ascii_case(p))
}
