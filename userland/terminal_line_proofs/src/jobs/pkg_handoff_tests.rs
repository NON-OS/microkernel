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

//! `pkg` asks the installer from a worker thread and the window thread picks
//! the answer up on its tick: one request at a time, the answer taken once,
//! and after Ctrl+C the worker's late answer discarded rather than handed to
//! a later request. The worker here is a real host thread.

mod ctrl_c;
mod race;
mod requests;

use crate::pkg_handoff::{Handoff, Look};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// An answer that counts how often it is dropped.
#[derive(Debug)]
struct Answer {
    n: u32,
    drops: Arc<AtomicUsize>,
}

impl PartialEq for Answer {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Drop for Answer {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

fn answer(n: u32, drops: &Arc<AtomicUsize>) -> Answer {
    Answer { n, drops: drops.clone() }
}

fn wait_ready<T>(slot: &Handoff<T>) -> T {
    loop {
        match slot.poll() {
            Look::Ready(v) => return v,
            Look::Waiting => thread::yield_now(),
            Look::Idle => panic!("the slot went idle with no answer"),
        }
    }
}
