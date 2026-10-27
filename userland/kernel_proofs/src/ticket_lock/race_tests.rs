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

use super::ticket::TicketMutex;
use super::{take, wait};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

#[test]
fn one_holder_at_a_time_across_threads() {
    // A plain read-modify-write under the lock: any overlap loses counts.
    let lock = Arc::new(TicketMutex::new(0u64));
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let lock = Arc::clone(&lock);
            thread::spawn(move || {
                for _ in 0..5_000 {
                    let mut held = take(&lock);
                    let seen = *held;
                    std::hint::spin_loop();
                    *held = seen + 1;
                }
            })
        })
        .collect();
    workers.into_iter().for_each(|w| w.join().unwrap());
    assert_eq!(*take(&lock), 4 * 5_000);
}

#[test]
fn a_loop_that_takes_again_cannot_keep_a_waiter_out() {
    // Hogs take, let go and take again as fast as they can, as the paging
    // manager's mapping loops do. Each hog has at most one number drawn
    // before the waiter's, so the waiter gets in after three holds at most.
    let lock = Arc::new(TicketMutex::new(()));
    let (stop, holds) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicU64::new(0)));
    let hogs: Vec<_> = (0..3)
        .map(|_| {
            let (lock, stop, holds) = (Arc::clone(&lock), Arc::clone(&stop), Arc::clone(&holds));
            thread::spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    let _held = take(&lock);
                    holds.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();
    let mut worst = 0;
    for _ in 0..500 {
        let number = lock.draw();
        let before = holds.load(Ordering::SeqCst);
        let _held = wait(&lock, number);
        worst = worst.max(holds.load(Ordering::SeqCst) - before);
    }
    stop.store(true, Ordering::Relaxed);
    hogs.into_iter().for_each(|h| h.join().unwrap());
    assert!(worst <= 3, "the waiter sat through {worst} holds");
}
