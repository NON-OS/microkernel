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

//! The ID3v2 tag at the start of an MP3: its header, and the walk over its
//! frames. Versions 2.2, 2.3 and 2.4 are read.
//!
//! The header is "ID3", a version, a flags byte and a "syncsafe" size: four
//! bytes of seven bits each, so the size itself never looks like an MPEG sync
//! word. Frames follow. In 2.2 a frame header is a 3-letter id and a 3-byte
//! size; in 2.3 a 4-letter id, a plain 4-byte size and two flag bytes; in 2.4
//! the same but with a syncsafe size. Some old iTunes builds wrote 2.4 tags
//! with plain sizes, so a 2.4 size with a high bit set is read as plain: a
//! syncsafe size never has one.
//!
//! "Unsynchronisation" puts a 0x00 after every 0xFF in the tag so no player
//! mistakes tag bytes for audio. In 2.2 and 2.3 the flag covers the whole tag,
//! so the tag is copied with those bytes taken out and read from the copy; in
//! 2.4 it is per frame and only that frame is copied. A cover read from a copy
//! has no place in the caller's buffer, so it is not reported: the text still
//! is, and such tags are rare.
//!
//! Every length here comes from the file and may lie. Nothing is indexed
//! without a check: a frame that runs past the bytes held (a truncated head,
//! or garbage) ends the walk, keeping what was read before it.

extern crate alloc;
use alloc::borrow::Cow;
use alloc::vec::Vec;

use super::frame::{Covers, Frame};
use super::Tags;

/// Reads the ID3v2 tag at the start of `head`; no tag gives empty tags.
pub fn read(head: &[u8]) -> Tags {
    let mut out = Tags::default();
    let Some(h) = head.get(..10) else {
        return out;
    };
    let ver = h[3];
    if &h[..3] != b"ID3" || !(2..=4).contains(&ver) {
        return out;
    }
    let flags = h[5];
    // 2.2 used this bit for a compression scheme it never defined.
    if ver == 2 && flags & 0x40 != 0 {
        return out;
    }
    let size = usize::try_from(syncsafe(&h[6..10])).unwrap_or(usize::MAX);
    let end = size.saturating_add(10).min(head.len());
    let raw = &head[10..end];
    let whole_unsync = flags & 0x80 != 0;
    let (body, base): (Cow<'_, [u8]>, Option<usize>) = if whole_unsync && ver < 4 {
        (Cow::Owned(resync(raw)), None)
    } else {
        (Cow::Borrowed(raw), Some(10))
    };
    let start = if ver >= 3 && flags & 0x40 != 0 { extended_len(&body, ver) } else { 0 };
    let walk = Walk { ver, base, unsync: whole_unsync && ver == 4 };
    let mut covers = Covers::default();
    walk.frames(&body, start, &mut out, &mut covers);
    out.cover = covers.best();
    out
}

/// How the frames of one tag are laid out.
struct Walk {
    ver: u8,
    /// Where the tag body starts in the caller's buffer, or `None` when the
    /// body is a resynchronised copy.
    base: Option<usize>,
    /// 2.4 with the tag-wide flag: every frame is unsynchronised.
    unsync: bool,
}

impl Walk {
    fn frames(&self, body: &[u8], mut pos: usize, out: &mut Tags, covers: &mut Covers) {
        let (id_len, hdr_len) = if self.ver == 2 { (3, 6) } else { (4, 10) };
        while let Some(fh) = pos.checked_add(hdr_len).and_then(|e| body.get(pos..e)) {
            let id = &fh[..id_len];
            // Padding (NULs) or garbage: either way the frames have ended.
            if !id.iter().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()) {
                return;
            }
            let size = match self.ver {
                2 => be(&fh[3..6]),
                3 => be(&fh[4..8]),
                _ => frame_size24(&fh[4..8]),
            };
            let flags = if self.ver == 2 { 0 } else { fh[9] };
            let data_at = pos + hdr_len;
            let Some(stop) = usize::try_from(size)
                .ok()
                .and_then(|s| data_at.checked_add(s))
                .filter(|&e| e <= body.len())
            else {
                return;
            };
            let frame = Frame {
                id,
                data: &body[data_at..stop],
                place: self.base.map(|b| b + data_at),
                ver: self.ver,
                flags,
                unsync: self.unsync,
            };
            frame.read(out, covers);
            pos = stop;
        }
    }
}

/// Skips the extended header: in 2.3 its size (plain) leaves out its own four
/// bytes, in 2.4 its size (syncsafe) counts them.
fn extended_len(body: &[u8], ver: u8) -> usize {
    let Some(b) = body.get(..4) else {
        return body.len();
    };
    let n = if ver == 3 { u64::from(be(b)) + 4 } else { u64::from(syncsafe(b)) };
    usize::try_from(n).unwrap_or(usize::MAX)
}

fn be(b: &[u8]) -> u32 {
    b.iter().fold(0u32, |n, &x| (n << 8) | u32::from(x))
}

fn syncsafe(b: &[u8]) -> u32 {
    b.iter().fold(0u32, |n, &x| (n << 7) | u32::from(x & 0x7F))
}

/// A 2.4 frame size: syncsafe, unless a high bit says a tagger wrote it plain.
fn frame_size24(b: &[u8]) -> u32 {
    if b.iter().any(|&x| x & 0x80 != 0) {
        be(b)
    } else {
        syncsafe(b)
    }
}

/// Undoes unsynchronisation: drops each 0x00 that follows a 0xFF.
pub fn resync(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len());
    let mut after_ff = false;
    for &b in raw {
        if !(after_ff && b == 0) {
            out.push(b);
        }
        after_ff = b == 0xFF;
    }
    out
}
