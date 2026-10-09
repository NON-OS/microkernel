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

use super::charset::Doc;
use super::mime::Mime;
use super::types::ContentKind;

/* A script runs only as a whole: a truncated one is refused. */
pub fn is_script(m: &Mime) -> bool {
    m.essence.ends_with(b"javascript") || m.essence.ends_with(b"ecmascript")
}

/* How the text is sniffed for its encoding. */
pub fn doc(kind: ContentKind, m: &Mime) -> Doc {
    match kind {
        ContentKind::Html => Doc::Html,
        _ if m.essence == b"text/css" => Doc::Css,
        _ => Doc::Plain,
    }
}

/* True when the first 512 bytes hold a byte no text has (a "binary data
byte" of MIME Sniffing 7.1) and no byte order mark says it is text:
an image or font sent without Content-Type. */
pub fn binary(b: &[u8]) -> bool {
    if b.starts_with(&[0xFE, 0xFF])
        || b.starts_with(&[0xFF, 0xFE])
        || b.starts_with(&[0xEF, 0xBB, 0xBF])
    {
        return false;
    }
    b.iter().take(512).any(|&c| matches!(c, 0x00..=0x08 | 0x0B | 0x0E..=0x1A | 0x1C..=0x1F))
}
