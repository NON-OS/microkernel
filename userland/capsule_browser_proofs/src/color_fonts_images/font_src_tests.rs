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

//! Which @font-face source the engine fetches.

use super::font_face_tests::faces;
use crate::browser::fonts::family_key;

#[test]
fn the_first_loadable_source_is_picked() {
    let k = family_key("S");
    let css =
        "@font-face{font-family:S;src:local(S),url(s.eot?#iefix) format('embedded-opentype'),\
               url(s.woff2) format('woff2'),url(s.ttf) format('truetype')}";
    assert_eq!(faces(css), [(k, "s.woff2".into())]);
    let data = "@font-face{font-family:D;src:url(data:font/woff2;base64,d09GMg==) format('woff2')}";
    assert_eq!(faces(data), [(family_key("D"), "data:font/woff2;base64,d09GMg==".into())]);
    let italic = "@font-face{font-family:I;src:url(i.woff2);font-style:italic}\
                  @font-face{font-family:I;src:url(r.woff2)}";
    assert_eq!(
        faces(italic),
        [(family_key("I"), "r.woff2".into())],
        "upright text takes the upright face"
    );
}
