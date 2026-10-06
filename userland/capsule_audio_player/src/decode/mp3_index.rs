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

//! Where each MP3 frame starts, learnt on the load's pass through the track.
//! An MP3 has no length in its header and no table of frames, so it played
//! with no duration and could not be seeked. The load already decodes the
//! whole track for its waveform; that pass records each frame's byte offset
//! and first sample, and gives the length when it reaches the end.

extern crate alloc;
use alloc::vec::Vec;

/// Frames decoded, and thrown away, before the one a seek lands in. Layer III
/// borrows up to 511 bytes from the frames before it (its bit reservoir), so
/// a decoder started cold on a frame can lose it; two before refill it.
pub const PRIME: usize = 2;

#[derive(Default)]
pub struct FrameIndex {
    /// (byte offset of the frame, its first sample frame), in file order.
    frames: Vec<(u32, u64)>,
    /// Sample frames seen so far on the pass.
    seen: u64,
    /// The whole track's length, once the pass reached its end.
    total: Option<u64>,
}

/// Where a seek starts decoding, and what it throws away to get there.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SeekPlan {
    /// Byte offset to start the priming decode at.
    pub prime_at: usize,
    /// Frames to decode and discard from there.
    pub prime: usize,
    /// Byte offset of the frame holding the target.
    pub frame_at: usize,
    /// Sample frames into that frame where the target lies.
    pub skip: u64,
}

impl FrameIndex {
    /// A frame of `samples` sample frames was decoded from byte `offset`.
    /// Only the first pass records; the replay after it adds nothing.
    pub fn record(&mut self, offset: usize, samples: u64) {
        if self.total.is_some() || offset > u32::MAX as usize {
            return;
        }
        if self.frames.last().is_some_and(|&(at, _)| at as usize >= offset) {
            return;
        }
        self.frames.push((offset as u32, self.seen));
        self.seen = self.seen.saturating_add(samples);
    }

    /// The pass reached the end of the data: the length is now known.
    pub fn finish(&mut self) {
        if self.total.is_none() && !self.frames.is_empty() {
            self.total = Some(self.seen);
        }
    }

    pub fn total(&self) -> Option<u64> {
        self.total
    }

    /// How to land on sample frame `target`, once the length is known.
    pub fn plan(&self, target: u64) -> Option<SeekPlan> {
        self.total?;
        let k = self.frames.partition_point(|&(_, first)| first <= target).checked_sub(1)?;
        let k0 = k.saturating_sub(PRIME);
        let (frame_at, first) = self.frames[k];
        Some(SeekPlan {
            prime_at: self.frames[k0].0 as usize,
            prime: k - k0,
            frame_at: frame_at as usize,
            skip: target - first,
        })
    }
}
