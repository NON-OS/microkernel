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

//! Music's tag reader. Each tag here is built byte by
//! byte in the test, so what is proven is the reader against the layout the
//! ID3 specifications give, not against a file some other tool wrote. The
//! reader must also never panic: a library scan reads whatever the user put
//! in their music folder, so every prefix of a tag and a stream of random
//! bytes are fed through it too.

use crate::audio_tags::{read_tags, Cover, CoverKind, Tags};

fn syncsafe(n: usize) -> [u8; 4] {
    [((n >> 21) & 0x7F) as u8, ((n >> 14) & 0x7F) as u8, ((n >> 7) & 0x7F) as u8, (n & 0x7F) as u8]
}

fn frame22(id: &[u8; 3], data: &[u8]) -> Vec<u8> {
    let n = data.len();
    let mut f = id.to_vec();
    f.extend_from_slice(&[(n >> 16) as u8, (n >> 8) as u8, n as u8]);
    f.extend_from_slice(data);
    f
}

fn frame23(id: &[u8; 4], data: &[u8]) -> Vec<u8> {
    frame23_flags(id, 0, data)
}

fn frame23_flags(id: &[u8; 4], flags: u8, data: &[u8]) -> Vec<u8> {
    let mut f = id.to_vec();
    f.extend_from_slice(&(data.len() as u32).to_be_bytes());
    f.extend_from_slice(&[0, flags]);
    f.extend_from_slice(data);
    f
}

fn frame24(id: &[u8; 4], data: &[u8]) -> Vec<u8> {
    frame24_flags(id, 0, data)
}

fn frame24_flags(id: &[u8; 4], flags: u8, data: &[u8]) -> Vec<u8> {
    let mut f = id.to_vec();
    f.extend_from_slice(&syncsafe(data.len()));
    f.extend_from_slice(&[0, flags]);
    f.extend_from_slice(data);
    f
}

fn tag(ver: u8, flags: u8, frames: &[Vec<u8>]) -> Vec<u8> {
    let body: Vec<u8> = frames.concat();
    let mut t = b"ID3".to_vec();
    t.extend_from_slice(&[ver, 0, flags]);
    t.extend_from_slice(&syncsafe(body.len()));
    t.extend_from_slice(&body);
    t
}

/// A text frame body: Latin-1 (encoding 0).
fn latin(s: &str) -> Vec<u8> {
    let mut v = vec![0u8];
    v.extend(s.chars().map(|c| c as u32 as u8));
    v
}

/// A text frame body: UTF-8 (encoding 3).
fn utf8(s: &str) -> Vec<u8> {
    let mut v = vec![3u8];
    v.extend_from_slice(s.as_bytes());
    v
}

/// A text frame body: UTF-16 with a byte order mark (encoding 1).
fn utf16_bom(s: &str, big: bool) -> Vec<u8> {
    let mut v = vec![1u8];
    v.extend_from_slice(if big { &[0xFE, 0xFF] } else { &[0xFF, 0xFE] });
    for u in s.encode_utf16() {
        v.extend_from_slice(&if big { u.to_be_bytes() } else { u.to_le_bytes() });
    }
    v
}

/// A text frame body: UTF-16 big endian, no mark (encoding 2).
fn utf16_be(s: &str) -> Vec<u8> {
    let mut v = vec![2u8];
    for u in s.encode_utf16() {
        v.extend_from_slice(&u.to_be_bytes());
    }
    v
}

const JPEG: &[u8] = &[0xFF, 0xD8, 0xFF, 0xE0, 1, 2, 3, 4, 5, 6];
const PNG: &[u8] = b"\x89PNG\r\n\x1a\nPNGDATA";

fn apic(mime: &str, kind: u8, desc: &str, img: &[u8]) -> Vec<u8> {
    let mut v = vec![0u8];
    v.extend_from_slice(mime.as_bytes());
    v.push(0);
    v.push(kind);
    v.extend_from_slice(desc.as_bytes());
    v.push(0);
    v.extend_from_slice(img);
    v
}

/// The bytes a reported cover points at.
fn cover_bytes<'a>(head: &'a [u8], c: &Cover) -> &'a [u8] {
    &head[c.at..c.at + c.len]
}

fn s(v: &Option<String>) -> Option<&str> {
    v.as_deref()
}

#[test]
fn v23_reads_every_text_field() {
    let t = tag(
        3,
        0,
        &[
            frame23(b"TIT2", &latin("Song")),
            frame23(b"TPE1", &latin("Band")),
            frame23(b"TALB", &latin("Record")),
            frame23(b"TPE2", &latin("Various")),
            frame23(b"TRCK", &latin("7/10")),
            frame23(b"TYER", &latin("1999")),
        ],
    );
    let got = read_tags(&t, None);
    assert_eq!(s(&got.title), Some("Song"));
    assert_eq!(s(&got.artist), Some("Band"));
    assert_eq!(s(&got.album), Some("Record"));
    assert_eq!(s(&got.album_artist), Some("Various"));
    assert_eq!(got.track, Some(7));
    assert_eq!(got.year, Some(1999));
    assert_eq!(got.cover, None);
}

#[test]
fn v24_reads_utf8_and_a_timestamp_year() {
    let t = tag(
        4,
        0,
        &[
            frame24(b"TIT2", &utf8("Jóga")),
            frame24(b"TPE1", &utf8("Björk")),
            frame24(b"TDRC", &utf8("2019-05-01")),
            frame24(b"TRCK", &utf8("3")),
        ],
    );
    let got = read_tags(&t, None);
    assert_eq!(s(&got.title), Some("Jóga"));
    assert_eq!(s(&got.artist), Some("Björk"));
    assert_eq!(got.year, Some(2019));
    assert_eq!(got.track, Some(3));
}

#[test]
fn all_four_text_encodings_decode() {
    let t = tag(
        3,
        0,
        &[
            frame23(b"TIT2", &latin("Caf\u{e9}")),
            frame23(b"TPE1", &utf16_bom("Sigur Rós", false)),
            frame23(b"TALB", &utf16_bom("Ágætis byrjun", true)),
            frame23(b"TPE2", &utf16_be("Ωmega")),
        ],
    );
    let got = read_tags(&t, None);
    assert_eq!(s(&got.title), Some("Café"));
    assert_eq!(s(&got.artist), Some("Sigur Rós"));
    assert_eq!(s(&got.album), Some("Ágætis byrjun"));
    assert_eq!(s(&got.album_artist), Some("Ωmega"));
    let t = tag(4, 0, &[frame24(b"TIT2", &utf8("日本語"))]);
    assert_eq!(s(&read_tags(&t, None).title), Some("日本語"));
}

#[test]
fn first_value_trimmed_and_blank_is_none() {
    let mut multi = utf16_bom("One", false);
    multi.extend_from_slice(&[0, 0, 0xFF, 0xFE, b'T', 0, b'w', 0, b'o', 0, 0, 0]);
    let t = tag(
        4,
        0,
        &[
            frame24(b"TIT2", &utf8("  Spaced  \0\0")),
            frame24(b"TPE1", &utf8("First\0Second")),
            frame24(b"TALB", &utf8("   \0")),
            frame24(b"TPE2", &multi),
            frame24(b"TRCK", &utf8("0")),
            frame24(b"TDRC", &utf8("19")),
        ],
    );
    let got = read_tags(&t, None);
    assert_eq!(s(&got.title), Some("Spaced"));
    assert_eq!(s(&got.artist), Some("First"));
    assert_eq!(got.album, None);
    assert_eq!(s(&got.album_artist), Some("One"));
    assert_eq!(got.track, None);
    assert_eq!(got.year, None);
}

#[test]
fn v22_frames_and_pic() {
    let mut pic = vec![0u8];
    pic.extend_from_slice(b"PNG");
    pic.push(3);
    pic.extend_from_slice(b"cover\0");
    pic.extend_from_slice(PNG);
    let t = tag(
        2,
        0,
        &[
            frame22(b"TT2", &latin("Old")),
            frame22(b"TP1", &latin("Artist")),
            frame22(b"TAL", &latin("Album")),
            frame22(b"TP2", &latin("AA")),
            frame22(b"TRK", &latin("12/12")),
            frame22(b"TYE", &latin("1988")),
            frame22(b"PIC", &pic),
        ],
    );
    let got = read_tags(&t, None);
    assert_eq!(s(&got.title), Some("Old"));
    assert_eq!(s(&got.artist), Some("Artist"));
    assert_eq!(s(&got.album), Some("Album"));
    assert_eq!(s(&got.album_artist), Some("AA"));
    assert_eq!(got.track, Some(12));
    assert_eq!(got.year, Some(1988));
    let c = got.cover.expect("cover");
    assert_eq!(c.mime, CoverKind::Png);
    assert_eq!(cover_bytes(&t, &c), PNG);
}

#[test]
fn front_cover_wins_over_an_earlier_picture() {
    for ver in [3u8, 4] {
        let f = |id: &[u8; 4], d: &[u8]| if ver == 3 { frame23(id, d) } else { frame24(id, d) };
        let t = tag(
            ver,
            0,
            &[
                f(b"TIT2", &latin("x")),
                f(b"APIC", &apic("image/png", 4, "back", PNG)),
                f(b"APIC", &apic("image/jpeg", 3, "front", JPEG)),
            ],
        );
        let c = read_tags(&t, None).cover.expect("cover");
        assert_eq!(c.mime, CoverKind::Jpeg);
        assert_eq!(c.len, JPEG.len());
        assert_eq!(cover_bytes(&t, &c), JPEG);
    }
}

#[test]
fn first_picture_when_no_front_cover_and_utf16_description() {
    let mut pic = vec![1u8];
    pic.extend_from_slice(b"image/x-odd\0");
    pic.push(0);
    pic.extend_from_slice(&[0xFF, 0xFE, b'd', 0, 0, 0]);
    pic.extend_from_slice(&[9, 9, 9]);
    let t = tag(3, 0, &[frame23(b"APIC", &pic), frame23(b"APIC", &apic("image/png", 5, "", PNG))]);
    let c = read_tags(&t, None).cover.expect("cover");
    assert_eq!(c.mime, CoverKind::Other);
    assert_eq!(cover_bytes(&t, &c), &[9, 9, 9]);
}

#[test]
fn mislabelled_picture_is_sniffed() {
    let t = tag(3, 0, &[frame23(b"APIC", &apic("image/png", 3, "", JPEG))]);
    let c = read_tags(&t, None).cover.expect("cover");
    assert_eq!(c.mime, CoverKind::Jpeg);
}

#[test]
fn truncated_head_keeps_fields_before_the_cut() {
    let t = tag(
        3,
        0,
        &[
            frame23(b"TIT2", &latin("Kept")),
            frame23(b"TPE1", &latin("Also kept")),
            frame23(b"TALB", &latin("Cut off here")),
            frame23(b"APIC", &apic("image/jpeg", 3, "", JPEG)),
        ],
    );
    let album_at = t.windows(4).position(|w| w == b"TALB").unwrap();
    let head = &t[..album_at + 14];
    let got = read_tags(head, None);
    assert_eq!(s(&got.title), Some("Kept"));
    assert_eq!(s(&got.artist), Some("Also kept"));
    assert_eq!(got.album, None);
    assert_eq!(got.cover, None);
}

#[test]
fn cover_past_the_head_is_not_reported() {
    let t = tag(
        3,
        0,
        &[frame23(b"TIT2", &latin("T")), frame23(b"APIC", &apic("image/jpeg", 3, "", JPEG))],
    );
    let got = read_tags(&t[..t.len() - 1], None);
    assert_eq!(s(&got.title), Some("T"));
    assert_eq!(got.cover, None);
}

fn sample_tags() -> Vec<Vec<u8>> {
    vec![
        tag(
            3,
            0,
            &[
                frame23(b"TIT2", &utf16_bom("Title", false)),
                frame23(b"TRCK", &latin("1/2")),
                frame23(b"APIC", &apic("image/jpeg", 3, "d", JPEG)),
            ],
        ),
        tag(
            4,
            0x40,
            &[
                vec![0, 0, 0, 6, 0, 0],
                frame24(b"TIT2", &utf8("Four")),
                frame24(b"APIC", &apic("image/png", 3, "", PNG)),
            ],
        ),
        tag(2, 0, &[frame22(b"TT2", &utf16_be("Two")), frame22(b"PIC", b"\0JPG\x03\0\xFF\xD8")]),
        tag(3, 0x80, &[frame23(b"TIT2", &[0, 0xFF, 0, b'a'])]),
    ]
}

#[test]
fn every_prefix_of_a_tag_is_safe() {
    for t in sample_tags() {
        for n in 0..=t.len() {
            let got = read_tags(&t[..n], Some(&t[..n]));
            if let Some(c) = got.cover {
                assert!(c.at + c.len <= n);
            }
        }
    }
}

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u8 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) as u8
    }
}

#[test]
fn random_bytes_never_panic() {
    let mut rng = Lcg(0x5EED);
    for round in 0..3000 {
        let len = usize::from(rng.next()) * 2 + usize::from(rng.next() % 8);
        let mut buf: Vec<u8> = (0..len).map(|_| rng.next()).collect();
        // Most rounds start with a plausible header so the frame walk, not
        // just the magic check, sees the garbage.
        if round % 4 != 0 && buf.len() >= 10 {
            buf[..3].copy_from_slice(b"ID3");
            buf[3] = 2 + rng.next() % 3;
            for b in &mut buf[6..10] {
                *b &= 0x7F;
            }
        }
        if round % 3 == 0 && buf.len() >= 128 {
            let at = buf.len() - 128;
            buf[at..at + 3].copy_from_slice(b"TAG");
        }
        let got = read_tags(&buf, Some(&buf));
        if let Some(c) = got.cover {
            assert!(c.at + c.len <= buf.len());
        }
    }
}

#[test]
fn mutated_tags_never_panic() {
    let mut rng = Lcg(42);
    for t in sample_tags() {
        for _ in 0..2000 {
            let mut m = t.clone();
            for _ in 0..1 + rng.next() % 4 {
                let i = usize::from(rng.next()) % m.len();
                m[i] = rng.next();
            }
            let got = read_tags(&m, None);
            if let Some(c) = got.cover {
                assert!(c.at + c.len <= m.len());
            }
        }
    }
}

#[test]
fn whole_tag_unsync_in_v23_is_resynchronised() {
    // The title's Latin-1 0xFF is followed by the inserted 0x00; the frame
    // size counts the bytes after resynchronisation (3: enc, 0xFF, 'a').
    let mut f = b"TIT2".to_vec();
    f.extend_from_slice(&[0, 0, 0, 3, 0, 0, 0, 0xFF, 0, b'a']);
    let t = tag(3, 0x80, &[f, frame23(b"APIC", &apic("image/png", 3, "", PNG))]);
    let got = read_tags(&t, None);
    assert_eq!(s(&got.title), Some("\u{ff}a"));
    // A cover read from the resynchronised copy has no place in `t`.
    assert_eq!(got.cover, None);
}

#[test]
fn v24_frame_unsync_and_length_indicator() {
    let mut data = vec![0, 0, 0, 3];
    data.extend_from_slice(&[0, 0xFF, 0, b'b']);
    let t = tag(4, 0, &[frame24_flags(b"TIT2", 0x03, &data)]);
    assert_eq!(s(&read_tags(&t, None).title), Some("\u{ff}b"));
}

#[test]
fn v24_plain_frame_size_from_old_itunes() {
    let text = "x".repeat(199);
    let mut f = b"TIT2".to_vec();
    f.extend_from_slice(&200u32.to_be_bytes());
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&latin(&text));
    let t = tag(4, 0, &[f, frame24(b"TPE1", &latin("After"))]);
    let got = read_tags(&t, None);
    assert_eq!(got.title.as_deref(), Some(text.as_str()));
    assert_eq!(s(&got.artist), Some("After"));
}

#[test]
fn extended_headers_are_skipped() {
    let ext23 = vec![0, 0, 0, 6, 0, 0, 0, 0, 0, 0];
    let t = tag(3, 0x40, &[ext23, frame23(b"TIT2", &latin("Three"))]);
    assert_eq!(s(&read_tags(&t, None).title), Some("Three"));
    let ext24 = vec![0, 0, 0, 6, 1, 0];
    let t = tag(4, 0x40, &[ext24, frame24(b"TIT2", &latin("Four"))]);
    assert_eq!(s(&read_tags(&t, None).title), Some("Four"));
}

#[test]
fn compressed_and_encrypted_frames_are_skipped() {
    let t = tag(
        3,
        0,
        &[
            frame23_flags(b"TIT2", 0x80, &latin("zipped")),
            frame23_flags(b"TPE1", 0x40, &latin("secret")),
            frame23_flags(b"TALB", 0x20, &[7, 0, b'G']),
            frame23(b"TIT2", &latin("Plain")),
        ],
    );
    let got = read_tags(&t, None);
    assert_eq!(s(&got.title), Some("Plain"));
    assert_eq!(got.artist, None);
    assert_eq!(s(&got.album), Some("G"));
}

#[test]
fn first_frame_of_a_kind_wins() {
    let t = tag(3, 0, &[frame23(b"TIT2", &latin("First")), frame23(b"TIT2", &latin("Second"))]);
    assert_eq!(s(&read_tags(&t, None).title), Some("First"));
}

#[test]
fn not_a_tag_reads_nothing() {
    assert_eq!(read_tags(b"", None), Tags::default());
    assert_eq!(read_tags(b"ID3", None), Tags::default());
    let mut t = tag(3, 0, &[frame23(b"TIT2", &latin("x"))]);
    t[3] = 5;
    assert_eq!(read_tags(&t, None), Tags::default());
    assert_eq!(read_tags(b"RIFF....WAVEfmt ", Some(b"short")), Tags::default());
}

fn v1(title: &str, artist: &str, album: &str, year: &str, track: Option<u8>) -> Vec<u8> {
    let mut b = vec![0u8; 128];
    b[..3].copy_from_slice(b"TAG");
    let put =
        |b: &mut Vec<u8>, at: usize, s: &str| b[at..at + s.len()].copy_from_slice(s.as_bytes());
    put(&mut b, 3, title);
    put(&mut b, 33, artist);
    put(&mut b, 63, album);
    put(&mut b, 93, year);
    put(&mut b, 97, "a comment");
    if let Some(n) = track {
        b[125] = 0;
        b[126] = n;
    }
    b[127] = 17;
    b
}

#[test]
fn v1_fallback_with_v11_track() {
    let mut tail = vec![0xAAu8; 300];
    tail.extend_from_slice(&v1("Old Title   ", "Old Artist", "Old Album", "1977", Some(4)));
    let got = read_tags(b"\xFF\xFBnot a tag", Some(&tail));
    assert_eq!(s(&got.title), Some("Old Title"));
    assert_eq!(s(&got.artist), Some("Old Artist"));
    assert_eq!(s(&got.album), Some("Old Album"));
    assert_eq!(got.year, Some(1977));
    assert_eq!(got.track, Some(4));
    assert_eq!(got.album_artist, None);
    let plain = v1("T", "A", "", "", None);
    let got = read_tags(b"", Some(&plain));
    assert_eq!(got.track, None);
    assert_eq!(got.album, None);
    assert_eq!(got.year, None);
}

#[test]
fn v1_latin1_maps_high_bytes() {
    let mut b = v1("", "", "", "", None);
    b[3] = 0xE9;
    assert_eq!(s(&read_tags(b"", Some(&b)).title), Some("é"));
}

#[test]
fn v2_wins_and_v1_fills_the_gaps() {
    let head = tag(3, 0, &[frame23(b"TIT2", &latin("New Title")), frame23(b"TRCK", &latin("9"))]);
    let tail = v1("Old Title", "Old Artist", "Old Album", "1977", Some(4));
    let got = read_tags(&head, Some(&tail));
    assert_eq!(s(&got.title), Some("New Title"));
    assert_eq!(got.track, Some(9));
    assert_eq!(s(&got.artist), Some("Old Artist"));
    assert_eq!(s(&got.album), Some("Old Album"));
    assert_eq!(got.year, Some(1977));
}
