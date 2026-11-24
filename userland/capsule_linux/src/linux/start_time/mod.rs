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

//! When one launch reached each of its stages, from the moment this capsule
//! began, said once as one `[LINUX-TIME] start` line when the run ends
//! (end.rs). It counts and times and names nothing: no
//! program, no path, no destination, so a private run says nothing here
//! about what ran or where it went. `up` is this capsule's start on the
//! kernel's uptime clock, the clock the spawn gate's [BENCH] lines use, so
//! the line joins to them.

mod end;
mod line;

pub use end::say;

use core::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// The program's bytes are in hand.
pub const READ: usize = 0;
/// The family's first network grant was answered by a front (net/grant).
pub const GRANT: usize = 1;
/// GTK's preparation is over, run or not.
pub const PREP: usize = 2;
/// The kernel made its guest: the family exists.
pub const FAMILY: usize = 3;
/// The program proved itself (start_guest::start).
pub const PROVED: usize = 4;
/// The program and its interpreter are in the guest's memory.
pub const LOADED: usize = 5;
/// The guest was let run.
pub const RUNNING: usize = 6;
/// The latest file the family mapped executable, a library most often.
pub const LIBS: usize = 7;
/// The first write to the run's standard output or error.
pub const OUT: usize = 8;
/// The first stream asked for outside the family.
pub const CONNECT: usize = 9;
/// The first byte such a stream brought in.
pub const BYTE: usize = 10;
/// How many stages there are, in the order the line says them.
pub const STAGES: usize = 11;

/// Each stage's milliseconds since the start, plus one: 0 is not reached.
static AT: [AtomicU64; STAGES] = [const { AtomicU64::new(0) }; STAGES];
/// Proofs asked for files the family mapped executable.
static MAPPED: AtomicU64 = AtomicU64::new(0);
/// The capsule's start on the uptime clock, plus one: 0 is not begun.
static BEGUN: AtomicU64 = AtomicU64::new(0);

fn now() -> u64 {
    u64::try_from(nonos_libc::mk_uptime_ms()).unwrap_or(0)
}

/// The capsule has begun; the first call in its run.
pub fn begin() {
    BEGUN.store(now().saturating_add(1), Relaxed);
}

fn since() -> Option<u64> {
    let begun = BEGUN.load(Relaxed).checked_sub(1)?;
    Some(now().saturating_sub(begun).saturating_add(1))
}

/// `stage` reached, the first time only.
pub fn reached(stage: usize) {
    if let (Some(at), Some(slot)) = (since(), AT.get(stage)) {
        let _ = slot.compare_exchange(0, at, Relaxed, Relaxed);
    }
}

/// A file the family mapped executable was proved now. Before the family
/// exists, these are GTK's tools, which are not the run's.
pub fn mapped() {
    if AT[FAMILY].load(Relaxed) == 0 {
        return;
    }
    MAPPED.fetch_add(1, Relaxed);
    if let Some(at) = since() {
        AT[LIBS].store(at, Relaxed);
    }
}
