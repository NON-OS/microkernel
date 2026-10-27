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

use super::read::find;
use super::write::{hand_over, publish, withdraw};
use super::TURN;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

/// What the writer below ever puts in a slot for `asid`: its root is fixed by
/// the asid and its owner is one of two pids. A reader that pairs one write's
/// asid with another's root or pid fails this.
fn whole(asid: u32, pid: u32, root: u64) -> bool {
    root == u64::from(asid) << 12 && (pid == asid ^ 0x55 || pid == asid ^ 0xAA)
}

#[test]
fn a_reader_never_sees_half_a_write() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    let stop = Arc::new(AtomicBool::new(false));
    let readers: Vec<_> = (0..3)
        .map(|_| {
            let stop = Arc::clone(&stop);
            thread::spawn(move || {
                let mut seen = 0u64;
                while !stop.load(Ordering::Relaxed) {
                    let mut check = |e: &(u32, u32, u64)| {
                        assert!(whole(e.0, e.1, e.2), "torn read {e:x?}");
                        seen += 1;
                        false
                    };
                    let _ = find(&mut check);
                    thread::yield_now();
                }
                seen
            })
        })
        .collect();
    // One writer, as the paging manager lock makes it: make, hand over and
    // take address spaces, a few alive at once, as fast as it can.
    for asid in 1u32..20_000 {
        publish(asid, asid ^ 0x55, u64::from(asid) << 12);
        assert!(hand_over(asid, asid ^ 0xAA));
        if asid > 8 {
            withdraw(asid - 8);
        }
    }
    (19_992u32..20_000).for_each(withdraw);
    stop.store(true, Ordering::Relaxed);
    let seen: u64 = readers.into_iter().map(|r| r.join().unwrap()).sum();
    assert!(seen > 0, "the readers never overlapped the writer");
}
