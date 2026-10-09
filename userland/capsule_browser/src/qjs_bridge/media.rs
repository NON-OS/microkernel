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

//! A script's matchMedia, judged as the page's own @media rules are.

use core::ffi::c_void;

use crate::browser::css::color::media_query_matches;
use crate::qjs_dom::{cstr, dom};

/// Whether the media query list `query` holds for the document's viewport:
/// 1 when it does, 0 when not.
///
/// matchMedia used to answer false to everything, so a page asking
/// `(min-width: 768px)` on a wide window was told it was narrow and drew
/// its phone layout, and one asking `(max-width: 767px)` disagreed with its
/// own stylesheet. The answer now comes from the same parser the
/// stylesheet's @media blocks are judged by, at the same viewport.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_media_matches(host: *mut c_void, query: *const u8) -> i32 {
    let (w, h) = dom(host).viewport;
    media_query_matches(&cstr(query), w, h) as i32
}
