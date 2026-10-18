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

//! The parts running on this thread's windows (`tests::part`), as the clock
//! sees them: the turns each has taken, and a lock each holds while it
//! answers, from reading the ring to its last write into it.

use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Weak};

#[derive(Default)]
pub struct Link {
    turns: AtomicU64,
    step: Mutex<()>,
}

impl Link {
    /// Held while the part answers.
    pub fn step(&self) -> MutexGuard<'_, ()> {
        self.step.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Another turn taken.
    pub fn turned(&self) {
        self.turns.fetch_add(1, Ordering::AcqRel);
    }
}

thread_local!(static LINKS: RefCell<Vec<Weak<Link>>> = const { RefCell::new(Vec::new()) });

/// A part about to run on a window this thread's driver uses; it counts as
/// running until its thread is joined and the link dropped.
pub fn attach() -> Arc<Link> {
    let link = Arc::new(Link::default());
    LINKS.with(|links| links.borrow_mut().push(Arc::downgrade(&link)));
    link
}

fn running() -> Vec<Arc<Link>> {
    LINKS.with(|links| {
        let mut links = links.borrow_mut();
        links.retain(|link| link.strong_count() > 0);
        links.iter().filter_map(Weak::upgrade).collect()
    })
}

/// The turns each running part has taken, in the order they were attached.
pub fn turns() -> Vec<u64> {
    running().iter().map(|link| link.turns.load(Ordering::Acquire)).collect()
}

/// Once no running part is midway through an answer.
pub fn between_steps() {
    for link in running() {
        drop(link.step());
    }
}
