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
use crate::decode::Decoder;
use crate::resample::{Resampler, OUT_RATE};
use super::defs::{FeedSink, State};

const STREAM_FORMAT: u16 = 0;
pub(super) const DECODE_FRAMES: usize = 512;

pub struct Transport {
    pub(super) state: State, pub(super) pos_frames: u64, pub(super) dur_frames: u64,
    pub(super) volume_q15: i32, pub(super) muted_q15: i32, pub(super) decoder: Option<Box<dyn Decoder>>,
    pub(super) resampler: Resampler, pub(super) client: Box<dyn FeedSink>,
    pub(super) scratch_src: Vec<i16>, pub(super) scratch_out: Vec<i16>,
    /// Why the last play stopped short of the end: the sink's failure.
    pub(super) fault: Option<&'static str>,
    /// Whether the sink holds a stream on the audio service. The service
    /// frees a stream only on close or when the owning process ends, and the
    /// boot instance never ends, so one left open is a slot lost for good.
    pub(super) stream: bool,
    /// The decoder's own sample rate. Positions count its frames, so times
    /// and second-sized seeks go through it; they went through the 48 kHz
    /// output rate, so a 44.1 kHz track showed and skipped the wrong time.
    pub(super) rate: u32,
    /// The loudest sample of the block last handed to the sink, after volume.
    pub(super) level: u16,
}

impl Transport {
    pub fn new(client: Box<dyn FeedSink>) -> Self {
        Self {
            state: State::Stopped, pos_frames: 0, dur_frames: 0, volume_q15: 1 << 15, muted_q15: 0,
            decoder: None, resampler: Resampler::new(OUT_RATE, 2), client,
            scratch_src: alloc::vec![0; DECODE_FRAMES * 2], scratch_out: Vec::new(),
            fault: None, stream: false, rate: OUT_RATE, level: 0,
        }
    }

    pub fn open(&mut self, dec: Box<dyn Decoder>) -> Result<(), &'static str> {
        let info = dec.info();
        self.release();
        self.client.open(STREAM_FORMAT)?;
        self.stream = true;
        self.resampler = Resampler::new(info.rate, info.channels);
        self.rate = info.rate.max(1);
        self.level = 0;
        self.dur_frames = info.total_frames.unwrap_or(0);
        self.pos_frames = 0;
        self.scratch_out.clear();
        self.scratch_src = alloc::vec![0; DECODE_FRAMES * info.channels.max(1) as usize];
        self.decoder = Some(dec);
        self.state = State::Stopped;
        self.fault = None;
        Ok(())
    }

    /// Play, or try again after a fault: a press of play is the user asking.
    pub fn play(&mut self) {
        if self.decoder.is_none() {
            return;
        }
        self.fault = None;
        // A stream given back once the queue ran out is taken again here,
        // fresh, so play after the end sounds as before.
        if !self.stream {
            if let Err(why) = self.client.open(STREAM_FORMAT) {
                self.state = State::Paused;
                self.fault = Some(why);
                return;
            }
            self.stream = true;
        }
        self.state = State::Playing;
        self.client.resume();
    }
    pub fn pause(&mut self) {
        self.state = State::Paused;
        self.client.pause();
    }
    pub fn stop(&mut self) {
        self.state = State::Stopped; self.pos_frames = 0;
        self.release(); self.decoder = None; self.scratch_out.clear();
    }
    /// Give the stream back to the audio service, keeping the track: play
    /// opens a fresh one. For a play that has ended with nothing after it.
    pub fn release(&mut self) {
        if self.stream {
            self.client.close();
            self.stream = false;
        }
    }
    pub fn seek_frames(&mut self, f: u64) -> bool {
        let target = f.min(self.dur_frames);
        if !self.decoder.as_mut().is_some_and(|d| d.seek(target)) {
            return false;
        }
        self.pos_frames = target;
        self.scratch_out.clear();
        true
    }
    pub fn set_volume(&mut self, q15: i32) {
        self.volume_q15 = q15.clamp(0, 1 << 15);
        self.muted_q15 = 0;
    }
    pub fn toggle_mute(&mut self) {
        let restore = self.muted_q15;
        self.muted_q15 = if restore == 0 { self.volume_q15.max(1) } else { 0 };
        self.volume_q15 = if restore == 0 { 0 } else { restore };
    }
    pub fn muted(&self) -> bool { self.muted_q15 != 0 }
    pub fn state(&self) -> State { self.state }
    pub fn pos_frames(&self) -> u64 { self.pos_frames }
    pub fn dur_frames(&self) -> u64 { self.dur_frames }
    pub fn volume_q15(&self) -> i32 { self.volume_q15 }
    pub fn fault(&self) -> Option<&'static str> { self.fault }
    /// `frames` of this track in milliseconds, at its own rate.
    pub fn frames_to_ms(&self, frames: u64) -> u32 {
        (frames.saturating_mul(1000) / self.rate.max(1) as u64).min(u32::MAX as u64) as u32
    }
    /// How many of this track's frames `secs` seconds is.
    pub fn secs_to_frames(&self, secs: u32) -> u64 { secs as u64 * self.rate as u64 }
    /// The output's level while it plays, 0 to 32767; 0 when it does not.
    pub fn level(&self) -> u16 {
        if self.state == State::Playing { self.level } else { 0 }
    }
}

/*
 * A window that closes drops its app, and with it this transport: its stream
 * goes back to the audio service then. Without this the boot instance, which
 * never exits, kept each closed window's stream, and the service allows a
 * process two, so the third window was refused sound.
 */
impl Drop for Transport {
    fn drop(&mut self) {
        self.release();
    }
}
