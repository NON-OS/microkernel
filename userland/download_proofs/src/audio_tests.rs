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

//! The audio check, on real MPEG frame headers and on what is not audio.

use nonos_download::{audio_start, Audio};

/// MPEG 1 layer III, 128 kbit/s, 44.1 kHz, no padding: 417 bytes a frame.
const HDR: [u8; 4] = [0xFF, 0xFB, 0x90, 0x64];
const FRAME: usize = 417;

fn frames(n: usize) -> Vec<u8> {
    let mut out = Vec::new();
    for _ in 0..n {
        let mut f = vec![0u8; FRAME];
        f[..4].copy_from_slice(&HDR);
        out.extend_from_slice(&f);
    }
    out
}

fn id3(size: usize, flags: u8) -> Vec<u8> {
    let mut t = vec![b'I', b'D', b'3', 4, 0, flags];
    for shift in [21, 14, 7, 0] {
        t.push(((size >> shift) & 0x7F) as u8);
    }
    t.extend(std::iter::repeat_n(b'x', size));
    t
}

#[test]
fn two_frames_back_to_back_are_an_mp3() {
    assert_eq!(audio_start(&frames(2), 0), Audio::Mp3 { frame_at: 0 });
}

#[test]
fn one_frame_with_no_second_where_it_ends_is_not() {
    let mut fake = frames(1);
    fake.extend_from_slice(b"<html>this is not the next frame</html>");
    assert!(matches!(audio_start(&fake, 0), Audio::Not(_)));
}

#[test]
fn sync_bits_with_impossible_fields_are_not_a_frame() {
    // Bit rate index 15, then sample rate index 3, then layer 0: each refused.
    for bad in [[0xFF, 0xFB, 0xF0, 0x64], [0xFF, 0xFB, 0x9C, 0x64], [0xFF, 0xF9, 0x90, 0x64]] {
        let mut b = bad.to_vec();
        b.extend(vec![0u8; 600]);
        assert!(matches!(audio_start(&b, 0), Audio::Not(_)), "{bad:02x?}");
    }
}

#[test]
fn an_id3_tag_then_frames_is_an_mp3_whose_audio_starts_after_the_tag() {
    let mut f = id3(300, 0);
    f.extend(frames(2));
    assert_eq!(audio_start(&f, 0), Audio::Mp3 { frame_at: 310 });
}

#[test]
fn a_tag_larger_than_what_came_asks_to_read_on_from_its_end() {
    // Album art: a 200 KB tag, and only the first 4 KB read so far.
    let mut f = id3(200_000, 0);
    f.extend(frames(2));
    assert_eq!(audio_start(&f[..4096], 0), Audio::ReadFrom(200_010));
    assert_eq!(audio_start(&f[200_010..], 200_010), Audio::Mp3 { frame_at: 200_010 });
}

#[test]
fn padding_after_a_tag_is_skipped_to_the_first_frame() {
    let mut f = id3(64, 0);
    f.extend(vec![0u8; 100]);
    f.extend(frames(2));
    assert_eq!(audio_start(&f, 0), Audio::Mp3 { frame_at: 174 });
}

#[test]
fn a_tag_with_nothing_audio_after_it_is_refused() {
    let mut f = id3(64, 0);
    f.extend_from_slice(b"<!DOCTYPE html><html></html>");
    assert!(matches!(audio_start(&f, 0), Audio::Not(_)));
}

#[test]
fn a_broken_tag_header_is_not_believed() {
    // Size bytes must each be under 0x80 (syncsafe).
    let mut f = vec![b'I', b'D', b'3', 3, 0, 0, 0x80, 0, 0, 0];
    f.extend(frames(2));
    assert!(matches!(audio_start(&f, 0), Audio::Not(_)));
}

#[test]
fn a_wav_file_plays_too() {
    let mut w = b"RIFF\x24\x00\x00\x00WAVEfmt ".to_vec();
    w.extend(vec![0u8; 32]);
    assert_eq!(audio_start(&w, 0), Audio::Wav);
}

#[test]
fn what_is_not_audio_is_named_for_the_sentence() {
    let cases: [(&[u8], &str); 6] = [
        (b"  <!DOCTYPE html><html><body>Not found</body></html>", "a web page"),
        (b"{\"error\":\"rate limited\"}", "a JSON answer"),
        (b"PK\x03\x04rest of a zip", "a ZIP archive"),
        (b"\xFF\xD8\xFF\xE0\x00\x10JFIF\x00", "an image"),
        (b"OggS\x00\x02\x00\x00", "audio or video in a format Music does not play (only MP3 and WAV)"),
        (b"plain text, nothing more", "something that is not MP3 audio"),
    ];
    for (bytes, said) in cases {
        assert_eq!(audio_start(bytes, 0), Audio::Not(said), "{:?}", String::from_utf8_lossy(bytes));
    }
}

#[test]
fn too_few_bytes_wait_for_more() {
    assert_eq!(audio_start(b"ID3", 0), Audio::Short);
    assert_eq!(audio_start(&[0xFF, 0xFB], 0), Audio::Short);
}

#[test]
fn mpeg2_and_layer_ii_frames_are_measured_right() {
    // MPEG 2 layer III, 64 kbit/s, 22.05 kHz: 72 * 64000 / 22050 = 208.
    let h2 = [0xFF, 0xF3, 0x80, 0x64];
    let mut b = Vec::new();
    for _ in 0..2 {
        let mut f = vec![0u8; 208];
        f[..4].copy_from_slice(&h2);
        b.extend(f);
    }
    assert_eq!(audio_start(&b, 0), Audio::Mp3 { frame_at: 0 });
    // MPEG 1 layer II, 192 kbit/s, 48 kHz: 144 * 192000 / 48000 = 576.
    let l2 = [0xFF, 0xFD, 0xA4, 0x00];
    let mut b = Vec::new();
    for _ in 0..2 {
        let mut f = vec![0u8; 576];
        f[..4].copy_from_slice(&l2);
        b.extend(f);
    }
    assert_eq!(audio_start(&b, 0), Audio::Mp3 { frame_at: 0 });
}
