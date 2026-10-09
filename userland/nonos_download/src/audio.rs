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
//! HTTP/1.1 for a client, with no I/O of its own.

//! Whether a downloaded file is audio Music can play, read from its first
//! bytes, before anything is handed to the player.
//!
//! Two bytes of 0xFF 0xE? are not enough: plenty of files that are not audio
//! have them somewhere near the start. An MP3 is an ID3v2 tag whose header
//! holds together, followed by an MPEG audio frame; or that frame at once. A
//! frame header is believed when its version, layer, bit rate and sample rate
//! are all real values, and, when the bytes reach that far, when a second
//! frame header begins exactly where the first frame ends.

/// What the first bytes of a file say.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audio {
    /// An MP3 whose first frame starts here.
    Mp3 { frame_at: usize },
    /// A WAV file, which Music also plays.
    Wav,
    /// An ID3 tag that ends at this offset, past the bytes given: read from
    /// there to find the first frame (album art makes tags large).
    ReadFrom(usize),
    /// Too few bytes yet to say.
    Short,
    /// Not audio; what it looks like instead, for the sentence.
    Not(&'static str),
}

/// Read `head`, the first bytes of the file (or of the file from `offset`
/// when the caller is following a `ReadFrom`).
pub fn audio_start(head: &[u8], offset: usize) -> Audio {
    if offset == 0 {
        if head.len() >= 12 && &head[0..4] == b"RIFF" && &head[8..12] == b"WAVE" {
            return Audio::Wav;
        }
        if head.starts_with(b"ID3") {
            let Some(end) = id3_end(head) else {
                return if head.len() < 10 { Audio::Short } else { Audio::Not(kind(head)) };
            };
            return frame_or_read(head, end);
        }
        if head.len() < 4 {
            return Audio::Short;
        }
    }
    match frame_len(head) {
        Some(len) => confirm(head, offset, len),
        None if head.len() < 4 => Audio::Short,
        // A tag can be followed by padding zeros before the first frame.
        None if offset > 0 => skip_padding(head, offset),
        None => Audio::Not(kind(head)),
    }
}

fn frame_or_read(head: &[u8], end: usize) -> Audio {
    if end + 4 > head.len() {
        return Audio::ReadFrom(end);
    }
    match audio_start(&head[end..], end) {
        Audio::Short => Audio::ReadFrom(end),
        other => other,
    }
}

fn skip_padding(head: &[u8], offset: usize) -> Audio {
    let zeros = head.iter().take_while(|&&b| b == 0).count();
    if zeros == 0 {
        return Audio::Not("a file with an ID3 tag but no MP3 audio after it");
    }
    if zeros + 4 > head.len() {
        return Audio::ReadFrom(offset + zeros);
    }
    match frame_len(&head[zeros..]) {
        Some(len) => confirm(&head[zeros..], offset + zeros, len),
        None => Audio::Not("a file with an ID3 tag but no MP3 audio after it"),
    }
}

/// A frame of `len` at the start of `head`: confirmed by the next header
/// when it is in reach, believed on its own when it is not.
fn confirm(head: &[u8], offset: usize, len: usize) -> Audio {
    if head.len() >= len + 4 && frame_len(&head[len..]).is_none() {
        return Audio::Not(kind(head));
    }
    Audio::Mp3 { frame_at: offset }
}

/// The end of an ID3v2 tag: a ten byte header, version 2 to 4, a syncsafe
/// size (each byte under 0x80), and a footer of ten more when flagged.
fn id3_end(head: &[u8]) -> Option<usize> {
    let h = head.get(..10)?;
    if !(2..=4).contains(&h[3]) || h[4] == 0xFF || h[6..10].iter().any(|&b| b >= 0x80) {
        return None;
    }
    let size = h[6..10].iter().fold(0usize, |acc, &b| (acc << 7) | b as usize);
    let footer = if h[3] == 4 && h[5] & 0x10 != 0 { 10 } else { 0 };
    Some(10 + size + footer)
}

/// The length of the MPEG audio frame whose header starts `head`, if it is a
/// real one: layers I, II and III of MPEG 1, 2 and 2.5.
fn frame_len(head: &[u8]) -> Option<usize> {
    let h = head.get(..4)?;
    if h[0] != 0xFF || h[1] & 0xE0 != 0xE0 {
        return None;
    }
    let version = (h[1] >> 3) & 3; // 0 = 2.5, 1 reserved, 2 = 2, 3 = 1
    let layer = (h[1] >> 1) & 3; // 1 = III, 2 = II, 3 = I, 0 reserved
    let rate_index = (h[2] >> 4) as usize;
    let sample_index = ((h[2] >> 2) & 3) as usize;
    let padding = ((h[2] >> 1) & 1) as usize;
    if version == 1 || layer == 0 || rate_index == 0 || rate_index == 15 || sample_index == 3 {
        return None;
    }
    let mpeg1 = version == 3;
    let kbps = bitrate(mpeg1, layer, rate_index)? as usize;
    let base = [44_100usize, 48_000, 32_000][sample_index];
    let sample_rate = match version {
        3 => base,
        2 => base / 2,
        _ => base / 4,
    };
    let len = match layer {
        3 => (12 * kbps * 1000 / sample_rate + padding) * 4,
        2 => 144 * kbps * 1000 / sample_rate + padding,
        _ if mpeg1 => 144 * kbps * 1000 / sample_rate + padding,
        _ => 72 * kbps * 1000 / sample_rate + padding,
    };
    (len >= 4).then_some(len)
}

fn bitrate(mpeg1: bool, layer: u8, index: usize) -> Option<u16> {
    const V1_L1: [u16; 15] = [0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448];
    const V1_L2: [u16; 15] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384];
    const V1_L3: [u16; 15] = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320];
    const V2_L1: [u16; 15] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256];
    const V2_L23: [u16; 15] = [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160];
    let table = match (mpeg1, layer) {
        (true, 3) => &V1_L1,
        (true, 2) => &V1_L2,
        (true, _) => &V1_L3,
        (false, 3) => &V2_L1,
        (false, _) => &V2_L23,
    };
    table.get(index).copied()
}

/// What a file that is not audio looks like, for "it starts like ...".
fn kind(head: &[u8]) -> &'static str {
    let lead: &[u8] = {
        let skip = head.iter().take_while(|b| b.is_ascii_whitespace()).count();
        &head[skip.min(head.len())..]
    };
    let lower = |p: &[u8]| lead.len() >= p.len() && lead[..p.len()].eq_ignore_ascii_case(p);
    if lower(b"<!doctype html") || lower(b"<html") || lower(b"<head") || lower(b"<body") {
        "a web page"
    } else if lead.starts_with(b"<") {
        "a markup document"
    } else if lead.starts_with(b"{") || lead.starts_with(b"[") {
        "a JSON answer"
    } else if head.starts_with(b"PK\x03\x04") {
        "a ZIP archive"
    } else if head.starts_with(b"\x89PNG") || head.starts_with(b"\xFF\xD8\xFF") || head.starts_with(b"GIF8") {
        "an image"
    } else if head.starts_with(b"%PDF") {
        "a PDF"
    } else if head.starts_with(b"OggS") || head.starts_with(b"fLaC") || (head.len() >= 8 && &head[4..8] == b"ftyp") {
        "audio or video in a format Music does not play (only MP3 and WAV)"
    } else {
        "something that is not MP3 audio"
    }
}
