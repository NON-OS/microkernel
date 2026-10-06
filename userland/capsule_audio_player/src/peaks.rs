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

//! A track's waveform built as it decodes, in memory that does not grow with
//! the track.
//!
//! The waveform used to be drawn from the whole track's PCM held at once:
//! about 42 MB for four minutes of stereo at 44.1 kHz, more than the player's
//! heap, so an ordinary song ran it out of memory. Only the loudest sample of
//! each stretch is kept here instead. When the stretches fill `MAX_PEAKS`,
//! neighbours merge and each stretch doubles, so a track of any length holds
//! at most `MAX_PEAKS` of them, still far finer than the bars drawn.

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

/// The most stretch peaks held at once, whatever the track's length.
pub const MAX_PEAKS: usize = 4096;

/// Samples in a stretch until the first merge.
const FIRST_STRETCH: usize = 64;

pub struct Peaks {
    stretch: usize,
    fill: usize,
    open: u16,
    peaks: Vec<u16>,
}

impl Default for Peaks {
    fn default() -> Self {
        Peaks { stretch: FIRST_STRETCH, fill: 0, open: 0, peaks: Vec::new() }
    }
}

impl Peaks {
    /// Fold in the next decoded samples, interleaved channels and all.
    pub fn push(&mut self, samples: &[i16]) {
        for &s in samples {
            self.open = self.open.max(s.unsigned_abs());
            self.fill += 1;
            if self.fill >= self.stretch {
                self.close();
            }
        }
    }

    fn close(&mut self) {
        self.peaks.push(self.open);
        self.open = 0;
        self.fill = 0;
        if self.peaks.len() >= MAX_PEAKS {
            let merged = self.peaks.chunks(2).map(|p| p.iter().copied().max().unwrap_or(0));
            self.peaks = merged.collect();
            self.stretch = self.stretch.saturating_mul(2);
        }
    }

    /// How many stretch peaks are held now, for the proofs of the bound.
    #[cfg(test)]
    pub fn held(&self) -> usize {
        self.peaks.len()
    }

    /// The waveform's `n_buckets` bars, each the loudest stretch under it,
    /// scaled so the loudest bar is 255. Silence, or no samples, is all zero.
    pub fn finish(mut self, n_buckets: usize) -> Vec<u8> {
        if self.fill > 0 {
            self.peaks.push(self.open);
        }
        let mut buckets = vec![0u8; n_buckets];
        let loudest = self.peaks.iter().copied().max().unwrap_or(0) as u32;
        let held = self.peaks.len();
        if loudest == 0 {
            return buckets;
        }
        for (i, bar) in buckets.iter_mut().enumerate() {
            let start = i * held / n_buckets;
            let end = ((i + 1) * held / n_buckets).clamp(start + 1, held);
            let peak = self.peaks[start..end].iter().copied().max().unwrap_or(0) as u32;
            *bar = (peak * 255 / loudest) as u8;
        }
        buckets
    }
}
