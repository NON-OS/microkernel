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

//! A swapchain's buffers as the client sees them. Pure: the fence is read
//! by the caller and handed in, so the host proofs drive every transition.
//!
//! A buffer the compositor shows, or may still be reading, is never handed
//! out to draw in: it is `Shown` until another is presented, then
//! `Retiring` until the compositor releases its fence (the surface's
//! release count moves past the one read when it was replaced).

use super::swapchain_damage::{Damage, Stale};

// Taking a first buffer on screen, and giving back one never shown.
#[path = "swapchain_turns.rs"]
mod turns;

/// Double or triple buffering; one buffer is a plain surface.
pub const MAX_BUFFERS: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Slot {
    /// The client may take it.
    Free,
    /// Taken by `acquire`, being drawn, not yet presented.
    Drawing,
    /// Presented: the compositor may read it at any time.
    Shown,
    /// Replaced on screen while its release count read `fence_was`.
    Retiring { fence_was: u32 },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Misuse {
    /// Presenting a buffer that was not taken to draw in.
    NotDrawing,
    /// A buffer index past the chain.
    NoSuchBuffer,
    /// Adopting a first buffer while one is already shown or drawn.
    AlreadyStarted,
}

pub struct Chain {
    slots: [Slot; MAX_BUFFERS],
    stale: [Stale; MAX_BUFFERS],
    count: usize,
}

impl Chain {
    /// `count` buffers, 2 or 3.
    pub fn new(count: usize) -> Option<Chain> {
        (2..=MAX_BUFFERS).contains(&count).then(|| Chain {
            slots: [Slot::Free; MAX_BUFFERS],
            stale: [Stale::Whole; MAX_BUFFERS],
            count,
        })
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn slot(&self, i: usize) -> Option<Slot> {
        self.slots[..self.count].get(i).copied()
    }

    /// A free buffer to draw in, and what of it is out of date: what other
    /// buffers presented since it was last shown, or all of it at first.
    /// None while every buffer is shown or retiring.
    pub fn acquire(&mut self) -> Option<(usize, Stale)> {
        let i = self.slots[..self.count].iter().position(|s| *s == Slot::Free)?;
        self.slots[i] = Slot::Drawing;
        Some((i, self.stale[i]))
    }

    /// Buffer `i` presented with `damage`; the one shown until now retires
    /// against `fence_of(that buffer)`, its release count read now.
    pub fn present(
        &mut self,
        i: usize,
        damage: Damage,
        fence_of: impl Fn(usize) -> u32,
    ) -> Result<(), Misuse> {
        match self.slot(i) {
            None => return Err(Misuse::NoSuchBuffer),
            Some(Slot::Drawing) => {}
            Some(_) => return Err(Misuse::NotDrawing),
        }
        for (j, s) in self.slots[..self.count].iter_mut().enumerate() {
            if *s == Slot::Shown {
                *s = Slot::Retiring { fence_was: fence_of(j) };
            }
        }
        self.slots[i] = Slot::Shown;
        for (j, st) in self.stale[..self.count].iter_mut().enumerate() {
            *st = if j == i { Stale::None } else { st.grow(damage) };
        }
        Ok(())
    }

    /// The release count of buffer `i` reads `fence`: a retiring buffer
    /// whose count moved is free again.
    pub fn fence(&mut self, i: usize, fence: u32) {
        if let Some(Slot::Retiring { fence_was }) = self.slot(i) {
            if fence != fence_was {
                self.slots[i] = Slot::Free;
            }
        }
    }

    /// The buffers waiting on a release, and the count each waits past.
    pub fn retiring(&self) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.slots[..self.count].iter().enumerate().filter_map(|(i, s)| match s {
            Slot::Retiring { fence_was } => Some((i, *fence_was)),
            _ => None,
        })
    }
}
