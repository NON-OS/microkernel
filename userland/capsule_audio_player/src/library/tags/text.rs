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

//! How the bytes of a tag's text become a `String`, for both tag versions.
//!
//! ID3v2 text frames start with one encoding byte: 0 is ISO-8859-1, 1 is
//! UTF-16 led by a byte order mark, 2 is UTF-16 big endian without one, 3 is
//! UTF-8. A frame may hold several values split by NULs (v2.4 lists several
//! artists that way) and taggers often pad with NULs or spaces, so only the
//! first value is kept, trimmed, and an empty one is `None`: a blank title is
//! no title, and the caller then shows the file's name instead.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Decodes one text value in the given ID3v2 encoding. An unknown encoding
/// byte is read as Latin-1, which maps every byte to some character and so
/// never fails.
pub fn decode(enc: u8, bytes: &[u8]) -> Option<String> {
    let s = match enc {
        1 => utf16(bytes, None),
        2 => utf16(bytes, Some(true)),
        3 => utf8(bytes),
        _ => latin1(bytes),
    };
    tidy(s)
}

/// ISO-8859-1: each byte is the code point of the same number, up to the
/// first NUL.
pub fn latin1(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    bytes[..end].iter().map(|&b| char::from(b)).collect()
}

fn utf8(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

/// UTF-16, big endian when `big` says so, otherwise by the byte order mark.
/// A value with no mark is read little endian, which is what the Windows
/// taggers that leave it out wrote.
fn utf16(bytes: &[u8], big: Option<bool>) -> String {
    let mut body = bytes;
    let mut be = big.unwrap_or(false);
    if big.is_none() {
        match body {
            [0xFE, 0xFF, rest @ ..] => {
                be = true;
                body = rest;
            }
            [0xFF, 0xFE, rest @ ..] => body = rest,
            _ => {}
        }
    }
    let units: Vec<u16> =
        body.chunks_exact(2)
            .map(|p| {
                if be {
                    u16::from_be_bytes([p[0], p[1]])
                } else {
                    u16::from_le_bytes([p[0], p[1]])
                }
            })
            .take_while(|&u| u != 0)
            .collect();
    char::decode_utf16(units).map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER)).collect()
}

/// Trims whitespace and stray NULs; an empty result is no value.
pub fn tidy(s: String) -> Option<String> {
    let t = s.trim_matches(|c: char| c.is_whitespace() || c == '\0');
    if t.is_empty() {
        None
    } else if t.len() == s.len() {
        Some(s)
    } else {
        Some(String::from(t))
    }
}

/// The number at the start of a track field: "3/12" is 3, " 07 " is 7. Zero,
/// or no digits, is no track: a track numbered 0 tells the listener nothing.
pub fn track_no(s: &str) -> Option<u16> {
    let t = s.trim();
    let digits = t.bytes().take_while(u8::is_ascii_digit).count();
    let n: u16 = t.get(..digits)?.parse().ok()?;
    (n != 0).then_some(n)
}

/// The year from the first four characters: "1999", or "2019-05-01" from a
/// v2.4 timestamp. Anything shorter, or not four digits, is no year.
pub fn year(s: &str) -> Option<u16> {
    let t = s.trim().as_bytes();
    let four = t.get(..4)?;
    if !four.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let n = four.iter().fold(0u16, |n, &d| n * 10 + u16::from(d - b'0'));
    (n != 0).then_some(n)
}
