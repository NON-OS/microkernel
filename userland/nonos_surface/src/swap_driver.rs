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

//! A window's swapchain: its buffers, already registered and shared with
//! the compositor, driven by `Chain`. A buffer is taken only once the
//! compositor released it (MkSurfaceFenceWait), and handed back as the
//! OP_PRESENT body the caller sends: a handle and a rectangle, never pixels.
//! The chain moves only once the compositor took the present, so a refused
//! one leaves the shown buffer shown.

use super::fence;
use super::swapchain::{Chain, MAX_BUFFERS};
use super::swapchain_damage::Stale;

// The present, built, sent by the caller, then settled.
#[path = "swap_present.rs"]
mod swap_present;
pub use swap_present::Pending;

pub struct Swap {
    chain: Chain,
    /// Each buffer's surface id (the project's) and handle (the compositor's).
    sids: [u64; MAX_BUFFERS],
    handles: [u64; MAX_BUFFERS],
    z: u32,
    /// Every pixel the client draws is opaque: its presents say so.
    opaque: bool,
}

impl Swap {
    /// `bufs` are (sid, handle) pairs, two or three, for the layer in band `z`.
    pub fn new(bufs: &[(u64, u64)], z: u32) -> Option<Swap> {
        let chain = Chain::new(bufs.len())?;
        let (mut sids, mut handles) = ([0; MAX_BUFFERS], [0; MAX_BUFFERS]);
        for (i, &(s, h)) in bufs.iter().enumerate() {
            (sids[i], handles[i]) = (s, h);
        }
        Some(Swap { chain, sids, handles, z, opaque: false })
    }

    /// The client draws every pixel opaque, so its presents say so and the
    /// compositor may scan a full-screen buffer out directly.
    pub fn set_opaque(&mut self, opaque: bool) {
        self.opaque = opaque;
    }

    /// Buffer `i` is the one the layer was submitted with, drawn before.
    pub fn adopt(&mut self, i: usize) -> bool {
        self.chain.adopt(i).is_ok()
    }

    /// The handle the compositor's layer shows, for every op that names
    /// the layer by its surface; buffer 0's before anything is shown.
    pub fn shown_handle(&self) -> u64 {
        self.handles[self.chain.shown().unwrap_or(0)]
    }

    pub fn count(&self) -> usize {
        self.chain.count()
    }

    /// The buffer the layer shows, once one was shown.
    pub fn shown(&self) -> Option<usize> {
        self.chain.shown()
    }

    /// A buffer to draw in and what of it is out of date, waiting up to
    /// `timeout_ms` for the compositor to release one. None when it did not.
    pub fn acquire(&mut self, timeout_ms: u64) -> Option<(usize, Stale)> {
        self.poll(0);
        if let Some(taken) = self.chain.acquire() {
            return Some(taken);
        }
        self.poll(timeout_ms);
        self.chain.acquire()
    }

    /// Read each retiring buffer's count; wait on the first up to
    /// `timeout_ms` when one is given.
    fn poll(&mut self, timeout_ms: u64) {
        let retiring: [Option<(usize, u32)>; MAX_BUFFERS] = {
            let mut r = [None; MAX_BUFFERS];
            self.chain.retiring().zip(r.iter_mut()).for_each(|(w, slot)| *slot = Some(w));
            r
        };
        for (n, (i, was)) in retiring.into_iter().flatten().enumerate() {
            let wait = if n == 0 { timeout_ms } else { 0 };
            if let Ok(now) = u32::try_from(fence::wait(self.sids[i], was, wait)) {
                self.chain.fence(i, now);
            }
        }
    }
}
