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

//! One ID3v2 frame: its flags, and which field of [`Tags`] it fills.
//!
//! A frame's two flag bytes can say its data is compressed or encrypted
//! (skipped: neither is worth a decompressor for a title), that a grouping
//! byte or a 4-byte length leads the data (stepped over), or in 2.4 that the
//! frame alone is unsynchronised (read from a copy). The first frame of each
//! kind wins, so a stray second TIT2 cannot rename a track.

extern crate alloc;
use alloc::borrow::Cow;

use super::id3v2::resync;
use super::picture;
use super::text::{decode, track_no, year};
use super::{Cover, Tags};

/// One frame as the walk found it.
pub struct Frame<'a> {
    pub id: &'a [u8],
    pub data: &'a [u8],
    /// Where `data` starts in the caller's buffer, `None` when the tag body
    /// is a resynchronised copy.
    pub place: Option<usize>,
    pub ver: u8,
    /// The second flag byte (format flags); 0 in 2.2, which has none.
    pub flags: u8,
    /// 2.4 with the tag-wide unsynchronisation flag set.
    pub unsync: bool,
}

/// The pictures seen so far: the first of any kind, and the first front
/// cover (picture type 3), which wins when there is one.
#[derive(Default)]
pub struct Covers {
    first: Option<Cover>,
    front: Option<Cover>,
}

impl Covers {
    pub fn best(&self) -> Option<Cover> {
        self.front.or(self.first)
    }
}

enum Field {
    Title,
    Artist,
    Album,
    AlbumArtist,
    Track,
    Year,
    Picture,
}

fn field(id: &[u8]) -> Option<Field> {
    Some(match id {
        b"TIT2" | b"TT2" => Field::Title,
        b"TPE1" | b"TP1" => Field::Artist,
        b"TALB" | b"TAL" => Field::Album,
        b"TPE2" | b"TP2" => Field::AlbumArtist,
        b"TRCK" | b"TRK" => Field::Track,
        b"TYER" | b"TDRC" | b"TYE" => Field::Year,
        b"APIC" | b"PIC" => Field::Picture,
        _ => return None,
    })
}

impl Frame<'_> {
    /// Reads this frame into `out`, or into `covers` for a picture.
    pub fn read(&self, out: &mut Tags, covers: &mut Covers) {
        let Some(kind) = field(self.id) else {
            return;
        };
        let Some((data, place)) = self.payload() else {
            return;
        };
        if let Field::Picture = kind {
            self.picture(&data, place, covers);
            return;
        }
        let Some((&enc, rest)) = data.split_first() else {
            return;
        };
        let Some(s) = decode(enc, rest) else {
            return;
        };
        match kind {
            Field::Title => fill(&mut out.title, || Some(s)),
            Field::Artist => fill(&mut out.artist, || Some(s)),
            Field::Album => fill(&mut out.album, || Some(s)),
            Field::AlbumArtist => fill(&mut out.album_artist, || Some(s)),
            Field::Track => fill(&mut out.track, || track_no(&s)),
            Field::Year => fill(&mut out.year, || year(&s)),
            Field::Picture => {}
        }
    }

    /// The frame's data with any grouping byte or length stepped over, and
    /// where it sits in the caller's buffer when it was not copied. `None`
    /// for compressed or encrypted frames, or flags that outrun the data.
    fn payload(&self) -> Option<(Cow<'_, [u8]>, Option<usize>)> {
        let f = self.flags;
        let (skip, unsync) = match self.ver {
            3 => {
                if f & 0xC0 != 0 {
                    return None;
                }
                (usize::from(f & 0x20 != 0), false)
            }
            4 => {
                if f & 0x0C != 0 {
                    return None;
                }
                let skip = usize::from(f & 0x40 != 0) + 4 * usize::from(f & 0x01 != 0);
                (skip, self.unsync || f & 0x02 != 0)
            }
            _ => (0, false),
        };
        let data = self.data.get(skip..)?;
        if unsync {
            Some((Cow::Owned(resync(data)), None))
        } else {
            Some((Cow::Borrowed(data), self.place.map(|p| p + skip)))
        }
    }

    fn picture(&self, data: &[u8], place: Option<usize>, covers: &mut Covers) {
        let Some(at) = place else {
            return;
        };
        let Some(pic) = picture::parse(data, self.ver == 2) else {
            return;
        };
        let cover = Cover { mime: pic.mime, at: at + pic.offset, len: data.len() - pic.offset };
        covers.first = covers.first.or(Some(cover));
        if pic.front {
            covers.front = covers.front.or(Some(cover));
        }
    }
}

/// Sets `slot` from `value` unless an earlier frame already did.
fn fill<T>(slot: &mut Option<T>, value: impl FnOnce() -> Option<T>) {
    if slot.is_none() {
        *slot = value();
    }
}
