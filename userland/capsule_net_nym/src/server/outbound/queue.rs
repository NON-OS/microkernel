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
//! What waits for the sending clock. A job is one message's frames, or one
//! resent fragment, with what to do once its last frame is on the wire; it
//! is not done before then, so a reply block count or an acknowledgement
//! record never runs ahead of the bytes. Pure, so the host proofs drive it.

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// Frames held at most. A sender that outruns the clock past this is told to
/// wait (E_BUSY) rather than held without bound: at the low rate this is
/// about a hundred seconds of sending.
pub const QUEUE_MAX_FRAMES: usize = 512;

/// A frame as it will be written: a Sphinx packet, sealed for the gateway's
/// session key at release, or bytes already framed for the older format.
pub enum Frame {
    Sphinx(Vec<u8>),
    Raw(Vec<u8>),
}

pub struct Job<D> {
    /// The client session the frames are for, told if they never leave.
    pub owner: Option<(u32, u32)>,
    pub frames: VecDeque<Frame>,
    pub done: D,
}

pub struct Queue<D> {
    jobs: VecDeque<Job<D>>,
    frames: usize,
}

impl<D> Queue<D> {
    pub const fn new() -> Self {
        Self { jobs: VecDeque::new(), frames: 0 }
    }

    /// Queue a job behind the others; handed back when it would not fit or
    /// carries no frame.
    pub fn push(&mut self, job: Job<D>) -> Result<(), Job<D>> {
        let n = job.frames.len();
        if n == 0 || self.frames.saturating_add(n) > QUEUE_MAX_FRAMES {
            return Err(job);
        }
        self.frames += n;
        self.jobs.push_back(job);
        Ok(())
    }

    /// The oldest frame, and its job's completion when that was the last.
    pub fn pop(&mut self) -> Option<(Frame, Option<(Option<(u32, u32)>, D)>)> {
        let front = self.jobs.front_mut()?;
        let frame = front.frames.pop_front()?;
        self.frames -= 1;
        if !front.frames.is_empty() {
            return Some((frame, None));
        }
        let job = self.jobs.pop_front()?;
        Some((frame, Some((job.owner, job.done))))
    }

    /// Everything still held, dropped at once: the link it was for is gone.
    pub fn drain(&mut self) -> Vec<Job<D>> {
        self.frames = 0;
        self.jobs.drain(..).collect()
    }
}
