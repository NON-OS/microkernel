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

//! The format label Now Playing shows: the format the decoder found in the
//! file and its rate, where every track used to read "WAV", MP3s included.

use crate::decode::wav::WavDecoder;
use crate::decode::{AudioInfo, Decoder};
use crate::track_fmt::format_of;

/// What an MP3 decoder reports: its rate from the first frame, no duration.
struct Mp3Like {
    rate: u32,
}

impl Decoder for Mp3Like {
    fn info(&self) -> AudioInfo {
        AudioInfo { rate: self.rate, channels: 2, total_frames: None }
    }
    fn kind(&self) -> &'static str {
        "MP3"
    }
    fn next(&mut self, _out: &mut [i16]) -> usize {
        0
    }
}

/// A RIFF/WAVE file of 16-bit stereo silence at `rate`.
fn wav(rate: u32) -> Vec<u8> {
    let data = vec![0u8; 64];
    let mut v = Vec::new();
    v.extend_from_slice(b"RIFF");
    v.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
    v.extend_from_slice(b"WAVEfmt ");
    v.extend_from_slice(&16u32.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes());
    v.extend_from_slice(&2u16.to_le_bytes());
    v.extend_from_slice(&rate.to_le_bytes());
    v.extend_from_slice(&(rate * 4).to_le_bytes());
    v.extend_from_slice(&4u16.to_le_bytes());
    v.extend_from_slice(&16u16.to_le_bytes());
    v.extend_from_slice(b"data");
    v.extend_from_slice(&(data.len() as u32).to_le_bytes());
    v.extend_from_slice(&data);
    v
}

#[test]
fn an_mp3_is_labelled_mp3_with_its_rate() {
    assert_eq!(format_of(&Mp3Like { rate: 44_100 }), "MP3 44100Hz");
    assert_eq!(format_of(&Mp3Like { rate: 48_000 }), "MP3 48000Hz");
}

#[test]
fn a_wav_is_labelled_wav_with_the_rate_in_its_header() {
    let dec = WavDecoder::new(wav(22_050)).expect("a wav");
    assert_eq!(format_of(&dec), "WAV 22050Hz");
    let dec = WavDecoder::new(wav(8_000)).expect("a wav");
    assert_eq!(format_of(&dec), "WAV 8000Hz");
}

#[test]
fn a_track_with_no_known_rate_is_labelled_by_its_format_alone() {
    assert_eq!(format_of(&Mp3Like { rate: 0 }), "MP3");
}
