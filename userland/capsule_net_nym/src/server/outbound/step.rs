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
//! One tick of the sender, pure: which packet leaves, and what came of it.
//! The gateway write and the cover packet come in as functions, so the host
//! proofs run this against a scripted gateway.

extern crate alloc;

use alloc::vec::Vec;

use super::queue::{Frame, Job, Queue};
use crate::mixnet::cover_clock::{Clocks, Profile, Tick};

type Owner = Option<(u32, u32)>;

pub enum Step<D> {
    /// Neither clock is due.
    Idle,
    /// A queued frame left; `done` is its job's completion when it was the last.
    Real { done: Option<(Owner, D)> },
    /// A cover packet left in the tick's place: a drop cover on a send
    /// tick, a loop cover on a loop tick.
    Cover(Tick),
    /// The tick was due but no cover could be built (no route, no gateway):
    /// it passes with nothing on the wire.
    NoCover(Tick),
    /// The write failed. The link is gone, so everything still queued is
    /// dropped with it; `owner` is the client of the frame that failed when
    /// its job was not among those still queued.
    Failed { code: u16, dropped: Vec<Job<D>>, owner: Owner },
}

pub fn step<D>(
    now: u64,
    profile: Profile,
    clocks: &mut Clocks,
    queue: &mut Queue<D>,
    random: u64,
    cover: impl FnOnce(Tick) -> Option<Vec<u8>>,
    write: impl FnOnce(&Frame) -> Result<(), u16>,
) -> Step<D> {
    let Some(tick) = clocks.take(now, profile, random) else {
        return Step::Idle;
    };
    if tick == Tick::Send {
        if let Some((frame, done)) = queue.pop() {
            return match write(&frame) {
                Ok(()) => Step::Real { done },
                Err(code) => {
                    let owner = done.and_then(|(owner, _)| owner);
                    Step::Failed { code, dropped: queue.drain(), owner }
                }
            };
        }
    }
    let Some(packet) = cover(tick) else {
        return Step::NoCover(tick);
    };
    match write(&Frame::Sphinx(packet)) {
        Ok(()) => Step::Cover(tick),
        Err(code) => Step::Failed { code, dropped: queue.drain(), owner: None },
    }
}
