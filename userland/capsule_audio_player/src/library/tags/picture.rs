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

//! The picture frame: APIC in ID3v2.3 and 2.4, PIC in 2.2.
//!
//! Its data is an encoding byte, the image format (a NUL-terminated MIME type
//! such as "image/jpeg" in APIC, a fixed three letters such as "JPG" in PIC),
//! a picture type byte (3 is the front cover), a description in the given
//! text encoding ending in a NUL (two NUL bytes for UTF-16), and then the
//! image itself up to the frame's end. Only the image's position is kept.
//!
//! The image's own first bytes are trusted over the declared format, since
//! taggers mislabel ("image/jpg", "jpeg", an empty type): a JPEG starts with
//! FF D8 and a PNG with 89 "PNG".

use super::CoverKind;

/// What a picture frame says about its image.
pub struct Picture {
    pub mime: CoverKind,
    /// The picture type is 3, the front cover.
    pub front: bool,
    /// Where the image bytes start inside the frame's data.
    pub offset: usize,
}

/// Parses a picture frame's data; `None` when it is malformed or holds no
/// image bytes.
pub fn parse(data: &[u8], v22: bool) -> Option<Picture> {
    let (&enc, rest) = data.split_first()?;
    let (fmt, rest) = if v22 {
        (rest.get(..3)?, rest.get(3..)?)
    } else {
        let n = rest.iter().position(|&b| b == 0)?;
        (&rest[..n], rest.get(n + 1..)?)
    };
    let (&kind, rest) = rest.split_first()?;
    let img = rest.get(description_len(enc, rest)?..)?;
    if img.is_empty() {
        return None;
    }
    Some(Picture {
        mime: sniff(img).unwrap_or_else(|| by_name(fmt)),
        front: kind == 3,
        offset: data.len() - img.len(),
    })
}

/// The description's length with its terminator: one NUL byte in the
/// single-byte encodings, an aligned pair in UTF-16.
fn description_len(enc: u8, rest: &[u8]) -> Option<usize> {
    if enc == 1 || enc == 2 {
        let i = rest.chunks_exact(2).position(|p| p == [0, 0])?;
        Some(i * 2 + 2)
    } else {
        Some(rest.iter().position(|&b| b == 0)? + 1)
    }
}

fn sniff(img: &[u8]) -> Option<CoverKind> {
    if img.starts_with(&[0xFF, 0xD8]) {
        Some(CoverKind::Jpeg)
    } else if img.starts_with(b"\x89PNG") {
        Some(CoverKind::Png)
    } else {
        None
    }
}

fn by_name(fmt: &[u8]) -> CoverKind {
    let is = |names: &[&[u8]]| names.iter().any(|n| fmt.eq_ignore_ascii_case(n));
    if is(&[b"image/jpeg", b"image/jpg", b"jpeg", b"jpg"]) {
        CoverKind::Jpeg
    } else if is(&[b"image/png", b"png"]) {
        CoverKind::Png
    } else {
        CoverKind::Other
    }
}
