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

use super::text::BOLD_KEY;

mod descriptor;
mod face;
mod matching;

use face::{face, Face};
use matching::best;

/* The faces to fetch for every @font-face in the sheet, as (slot key,
 * source url). Text draws in two cuts per family: the regular slot takes
 * the face CSS weight matching picks for 400, so a 500 cut serves only
 * when no 400 one exists, and the bold slot (BOLD_KEY set) the face it
 * picks for 700, if that face's lightest weight is 600 or more. A
 * variable face spanning both stays out of the bold slot: its wght axis
 * is not applied, so bold text thickens the regular cut instead. Only
 * sources the rasterizer can load are picked. */
pub fn collect_font_faces(css: &str) -> Vec<(u32, String)> {
    let mut faces: Vec<Face> = Vec::new();
    let mut rest = css;
    while let Some(pos) = rest.find("@font-face") {
        rest = &rest[pos + "@font-face".len()..];
        let Some(open) = rest.find('{') else { break };
        let Some(close) = rest[open..].find('}') else { break };
        let body = &rest[open + 1..open + close];
        rest = &rest[open + close + 1..];
        faces.extend(face(body));
    }
    let mut out = Vec::new();
    for (i, first) in faces.iter().enumerate() {
        if faces[..i].iter().any(|f| f.key == first.key) {
            continue;
        }
        let family: Vec<&Face> = faces.iter().filter(|f| f.key == first.key).collect();
        if let Some(f) = best(&family, 400) {
            out.push((first.key, f.url.clone()));
        }
        if let Some(f) = best(&family, 700).filter(|f| f.weight.0 >= 600) {
            out.push((first.key | BOLD_KEY, f.url.clone()));
        }
    }
    out
}
