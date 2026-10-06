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

//! Loading a track in bounded memory: the waveform keeps stretch peaks rather
//! than the whole decoded track, a decoder rewinds so the file is held once,
//! and a file past the limit is refused rather than cut short.

use crate::audio_trouble::track_trouble;
use crate::decode::wav::WavDecoder;
use crate::decode::Decoder;
use crate::peaks::{Peaks, MAX_PEAKS};
use crate::track_limit::{refuse_size, MAX_FILE, READ_LIMIT, TOO_LARGE};

/// Ten minutes of stereo at 48 kHz, fed in the loader's 4096-sample chunks.
#[test]
fn a_long_track_holds_no_more_than_the_peak_bound() {
    let chunk = [1200i16; 4096];
    let mut peaks = Peaks::default();
    let total = 10 * 60 * 48_000 * 2;
    let mut fed = 0;
    while fed < total {
        peaks.push(&chunk);
        fed += chunk.len();
        assert!(peaks.held() <= MAX_PEAKS);
    }
    let bars = peaks.finish(96);
    assert_eq!(bars.len(), 96);
    assert!(bars.iter().all(|&b| b == 255));
}

#[test]
fn silence_and_nothing_draw_flat() {
    let mut quiet = Peaks::default();
    quiet.push(&[0i16; 10_000]);
    assert!(quiet.finish(96).iter().all(|&b| b == 0));
    let none = Peaks::default().finish(96);
    assert_eq!(none.len(), 96);
    assert!(none.iter().all(|&b| b == 0));
}

#[test]
fn a_loud_passage_lands_where_it_plays() {
    // Quiet first half, loud second half, long enough to have merged.
    let mut peaks = Peaks::default();
    for _ in 0..200 {
        peaks.push(&[100i16; 4096]);
    }
    for _ in 0..200 {
        peaks.push(&[i16::MIN; 4096]);
    }
    let bars = peaks.finish(96);
    assert!(bars[..47].iter().all(|&b| b < 10));
    assert!(bars[49..].iter().all(|&b| b == 255));
}

#[test]
fn a_track_shorter_than_one_stretch_still_draws() {
    let mut peaks = Peaks::default();
    peaks.push(&[0, 300, -600]);
    let bars = peaks.finish(96);
    assert!(bars.iter().all(|&b| b == 255));
}

/// A RIFF/WAVE file of 16-bit mono samples.
fn wav(samples: &[i16]) -> Vec<u8> {
    let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    let mut v = Vec::new();
    v.extend_from_slice(b"RIFF");
    v.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
    v.extend_from_slice(b"WAVEfmt ");
    v.extend_from_slice(&16u32.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes());
    v.extend_from_slice(&8000u32.to_le_bytes());
    v.extend_from_slice(&16000u32.to_le_bytes());
    v.extend_from_slice(&2u16.to_le_bytes());
    v.extend_from_slice(&16u16.to_le_bytes());
    v.extend_from_slice(b"data");
    v.extend_from_slice(&(data.len() as u32).to_le_bytes());
    v.extend_from_slice(&data);
    v
}

fn read_all(dec: &mut dyn Decoder) -> Vec<i16> {
    let mut out = Vec::new();
    let mut buf = [0i16; 3];
    loop {
        let n = dec.next(&mut buf);
        if n == 0 {
            return out;
        }
        out.extend_from_slice(&buf[..n]);
    }
}

#[test]
fn a_decoder_read_through_for_the_waveform_rewinds_to_play_it_all() {
    let samples = [1i16, -2, 3, -4, 5, -6, 7];
    let mut dec = WavDecoder::new(wav(&samples)).expect("a wav");
    assert_eq!(read_all(&mut dec), samples);
    assert!(read_all(&mut dec).is_empty());
    assert!(dec.rewind());
    assert_eq!(read_all(&mut dec), samples);
}

#[test]
fn a_read_at_the_track_limit_tells_a_cut_off_file_from_one_that_fits() {
    assert_eq!(READ_LIMIT as u64, MAX_FILE as u64 + 1);
    assert_eq!(refuse_size(MAX_FILE as u64), None);
    assert_eq!(refuse_size(READ_LIMIT as u64), Some(TOO_LARGE));
    assert_eq!(refuse_size(u64::MAX), Some(TOO_LARGE));
    assert_eq!(refuse_size(0), None);
}

#[test]
fn a_file_past_the_limit_says_so_in_the_bar() {
    assert_eq!(track_trouble(TOO_LARGE), "Cannot play: the file is larger than 32 MiB");
}
