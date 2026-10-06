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

//! What Music reads from an MP3's tags: title, artist, album, album artist,
//! track and year, and where the cover picture sits in the bytes it was given.
//!
//! The library used to show a file's own name as its artist, because nothing
//! here could read a tag. This tree is that reader. It is pure: it takes the
//! bytes the caller already holds (the first part of the file, and its last
//! 128 bytes) and never touches the filesystem, so the host proofs run the very
//! source the capsule builds.
//!
//! An ID3v2 tag at the start of the file is read first. An ID3v1 block at the
//! end only fills the fields the v2 tag left empty, since v1 is a fixed
//! 30-byte Latin-1 record that cuts long names short and v2 is what every
//! modern tagger writes. Neither reader can panic, whatever the bytes: a file
//! is user data, and a library scan must not take the player down over one
//! broken tag.

extern crate alloc;
use alloc::string::String;

mod frame;
mod id3v1;
mod id3v2;
mod picture;
mod text;

/// The fields Music shows for a track. Each one is `None` when no tag named
/// it, so the caller can fall back to the file's name rather than show an
/// empty line.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tags {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub track: Option<u16>,
    pub year: Option<u16>,
    pub cover: Option<Cover>,
}

/// Where the cover picture's bytes lie inside the `head` slice handed to
/// [`read_tags`]: `head[at..at + len]`. An offset rather than a copy, since a
/// cover is often a few hundred KiB and most tracks in a scan never have
/// their cover drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cover {
    pub mime: CoverKind,
    pub at: usize,
    pub len: usize,
}

/// The picture's format, so the caller picks a decoder without sniffing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverKind {
    Jpeg,
    Png,
    Other,
}

/// Reads the tags of one file. `head` is the start of the file, which may be
/// cut short (a reader may load only the first 256 KiB); fields past the cut
/// are simply not found. `tail` is the end of the file (its last 128 bytes
/// are looked at), or `None` when the caller did not read it.
pub fn read_tags(head: &[u8], tail: Option<&[u8]>) -> Tags {
    let mut tags = id3v2::read(head);
    if let Some(tail) = tail {
        let old = id3v1::read(tail);
        // v1 fills gaps only: v2 is longer, richer and written later by any
        // tagger that writes both, so where they disagree v2 is right.
        tags.title = tags.title.or(old.title);
        tags.artist = tags.artist.or(old.artist);
        tags.album = tags.album.or(old.album);
        tags.track = tags.track.or(old.track);
        tags.year = tags.year.or(old.year);
    }
    tags
}
