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

use alloc::borrow::Cow;
use alloc::string::String;

use super::newlines::push_normalized;

/// The bytes of a page as the text the tokenizer reads (13.2.3.5).
///
/// The HTTP layer hands over UTF-8, having already converted any legacy
/// encoding. A byte that is not valid UTF-8 still arrives now and then, and
/// refusing the whole document for it left a blank page where the browser
/// next door shows the text with one odd character. Each maximal invalid
/// sequence becomes one U+FFFD, the WHATWG decoder's rule. A leading byte
/// order mark is dropped, and CR LF pairs and lone CRs become LF.
///
/// Valid UTF-8 without a CR, which is most pages, is read where it lies
/// rather than copied.
pub fn decode(bytes: &[u8]) -> Cow<'_, str> {
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    if let Ok(text) = core::str::from_utf8(bytes) {
        if !bytes.contains(&b'\r') {
            return Cow::Borrowed(text);
        }
    }
    /* Sized first: three bytes per replacement, so the string never regrows. */
    let size =
        bytes.utf8_chunks().map(|c| c.valid().len() + 3 * usize::from(!c.invalid().is_empty()));
    let mut out = String::with_capacity(size.sum());
    for chunk in bytes.utf8_chunks() {
        push_normalized(&mut out, chunk.valid());
        if !chunk.invalid().is_empty() {
            out.push('\u{FFFD}');
        }
    }
    Cow::Owned(out)
}
