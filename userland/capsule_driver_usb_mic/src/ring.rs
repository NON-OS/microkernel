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
//! What was captured and not yet read. A client that falls behind loses
//! the oldest frames, whole, and is told how many bytes it lost: the
//! device does not wait, so something must give, and the newest sound is
//! the one worth keeping.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// About a third of a second of 48 kHz stereo.
pub const RING_CAP: usize = 64 * 1024;

#[derive(Default)]
pub struct Ring {
    q: VecDeque<u8>,
    /// Bytes per frame: 2 a channel.
    pub frame: usize,
    pub overrun: u32,
}

impl Ring {
    pub fn new(frame: usize) -> Ring {
        Ring { q: VecDeque::new(), frame: frame.max(1), overrun: 0 }
    }

    /// `b` kept, at most RING_CAP in all; older whole frames go to make room.
    pub fn put(&mut self, b: &[u8]) {
        let cut = b.len().saturating_sub(RING_CAP - RING_CAP % self.frame);
        let b = &b[cut..];
        let over = (self.q.len() + b.len()).saturating_sub(RING_CAP);
        let drop = (over.div_ceil(self.frame) * self.frame).min(self.q.len());
        self.q.drain(..drop);
        self.overrun = self.overrun.saturating_add((cut + drop) as u32);
        self.q.extend(b);
    }

    /// One isochronous packet's bytes. A packet carries whole audio frames
    /// (USB Audio 2.0 2.3.1.1, as recalled, unverified); bytes past the last
    /// whole one are the device's error, dropped and counted as lost, so no
    /// later frame is read a channel out of step.
    pub fn put_packet(&mut self, p: &[u8]) {
        let tail = p.len() % self.frame;
        self.overrun = self.overrun.saturating_add(tail as u32);
        self.put(&p[..p.len() - tail]);
    }

    /// Up to `most` bytes of whole frames, and the overrun since the last.
    pub fn take(&mut self, most: usize) -> (u32, Vec<u8>) {
        let n = most.min(self.q.len());
        let n = n - n % self.frame;
        (core::mem::take(&mut self.overrun), self.q.drain(..n).collect())
    }
}
