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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::url::{join, Url};

/* Queue the web fonts `css` declares (`seen` holds the families met
 * before); each face is fetched once and text lays out again with its
 * real metrics when it lands. A data: source, the way icon fonts ship,
 * carries its bytes inline and installs on the spot, before the layout
 * that follows measures with it. Run when the CSS text changed, the only
 * time a new face can appear. */
pub(super) fn queue_faces(
    seen: &mut Vec<u32>,
    queue: &mut Vec<(u32, String)>,
    base: Option<&Url>,
    css: &str,
) {
    for (key, src) in crate::browser::fonts::collect_font_faces(css) {
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);
        if src.starts_with("data:") {
            if let Some(bytes) = crate::browser::image::data_uri_bytes(&src) {
                let _ = crate::browser::fonts::ingest_font(key, bytes);
            }
        } else if let Some(base) = base {
            queue.push((key, join(base, &src)));
        }
    }
}
