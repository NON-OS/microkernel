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

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use super::decoder::{AudioInfo, Decoder};
use super::minimp3_sys::mp3dec_t;
use super::mp3_frame::{decode_frame, empty_frame_info, new_decoder, MAX_SAMPLES};
use super::mp3_index::FrameIndex;

pub struct Mp3Decoder {
    data: Vec<u8>,
    dec: Box<mp3dec_t>,
    cursor: usize,
    rate: u32,
    channels: u8,
    scratch: Vec<i16>,
    scratch_pos: usize,
    /// Each frame's offset and first sample, from the load's pass, which
    /// gives the length and lets a seek land mid-track (`mp3_index.rs`).
    index: FrameIndex,
    /// The pass has decoded through to the end of the data.
    at_end: bool,
}

impl Mp3Decoder {
    pub fn new(data: Vec<u8>) -> Result<Mp3Decoder, &'static str> {
        let mut dec = new_decoder();
        let mut pcm = [0i16; MAX_SAMPLES];
        let mut scratch = Vec::new();
        let mut cursor = 0usize;
        let mut rate = 0u32;
        let mut channels = 0u8;
        let mut index = FrameIndex::default();
        while cursor < data.len() {
            let mut fi = empty_frame_info();
            let ret = decode_frame(&mut dec, &data[cursor..], &mut pcm, &mut fi);
            if ret > 0 {
                index.record(cursor, ret as u64);
            }
            cursor += fi.frame_bytes as usize;
            if ret > 0 {
                rate = fi.hz as u32;
                channels = fi.channels as u8;
                scratch.extend_from_slice(&pcm[..ret as usize * fi.channels as usize]);
                break;
            }
            if fi.frame_bytes == 0 {
                break;
            }
        }
        if rate == 0 || channels == 0 {
            return Err("no mp3 frame");
        }
        Ok(Mp3Decoder {
            data,
            dec,
            cursor,
            rate,
            channels,
            scratch,
            scratch_pos: 0,
            index,
            at_end: false,
        })
    }
}

impl Decoder for Mp3Decoder {
    fn info(&self) -> AudioInfo {
        AudioInfo { rate: self.rate, channels: self.channels, total_frames: self.index.total() }
    }

    fn kind(&self) -> &'static str {
        "MP3"
    }

    fn next(&mut self, out: &mut [i16]) -> usize {
        let mut pcm = [0i16; MAX_SAMPLES];
        let mut written = 0;
        while written < out.len() {
            if self.scratch_pos < self.scratch.len() {
                let take = (out.len() - written).min(self.scratch.len() - self.scratch_pos);
                out[written..written + take]
                    .copy_from_slice(&self.scratch[self.scratch_pos..self.scratch_pos + take]);
                self.scratch_pos += take;
                written += take;
                continue;
            }
            self.scratch.clear();
            self.scratch_pos = 0;
            if self.cursor >= self.data.len() {
                self.at_end = true;
                break;
            }
            let mut fi = empty_frame_info();
            let ret = decode_frame(&mut self.dec, &self.data[self.cursor..], &mut pcm, &mut fi);
            if ret > 0 {
                self.index.record(self.cursor, ret as u64);
            }
            self.cursor += fi.frame_bytes as usize;
            if ret > 0 {
                self.scratch.extend_from_slice(&pcm[..ret as usize * fi.channels as usize]);
            } else if fi.frame_bytes == 0 {
                self.at_end = true;
                break;
            }
        }
        written
    }

    // A fresh minimp3 state at byte zero decodes the stream exactly as `new`
    // did, skipping the same tag bytes on the way to the first frame.
    // Started cold a frame or two before the target so the bit reservoir is
    // full again, then into the target's frame, its earlier samples skipped.
    fn seek(&mut self, frame: u64) -> bool {
        let Some(plan) = self.index.plan(frame) else { return false };
        self.dec = new_decoder();
        self.scratch.clear();
        self.scratch_pos = 0;
        let mut pcm = [0i16; MAX_SAMPLES];
        let mut at = plan.prime_at;
        for _ in 0..plan.prime {
            let Some(rest) = self.data.get(at..) else { break };
            let mut fi = empty_frame_info();
            decode_frame(&mut self.dec, rest, &mut pcm, &mut fi);
            if fi.frame_bytes == 0 {
                break;
            }
            at += fi.frame_bytes as usize;
        }
        self.cursor = plan.frame_at;
        let Some(rest) = self.data.get(self.cursor..) else { return false };
        let mut fi = empty_frame_info();
        let ret = decode_frame(&mut self.dec, rest, &mut pcm, &mut fi);
        self.cursor += fi.frame_bytes as usize;
        if ret > 0 {
            let ch = fi.channels.max(1) as usize;
            self.scratch.extend_from_slice(&pcm[..ret as usize * ch]);
            self.scratch_pos = (plan.skip as usize).saturating_mul(ch).min(self.scratch.len());
        }
        true
    }

    // The first pass through ends here: what it saw is the track's length.
    fn rewind(&mut self) -> bool {
        if self.at_end {
            self.index.finish();
        }
        self.dec = new_decoder();
        self.cursor = 0;
        self.scratch.clear();
        self.scratch_pos = 0;
        true
    }
}
