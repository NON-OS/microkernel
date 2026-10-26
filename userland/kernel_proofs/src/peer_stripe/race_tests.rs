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

use super::stripe::{hold, stripe_of, STRIPES};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering::SeqCst};
use std::sync::{Arc, Mutex};
use std::thread;

const SPACES: usize = 20_000;

struct Machine {
    locks: Vec<Mutex<()>>,
    named: AtomicU32,
    alive: Vec<AtomicBool>,
    used: AtomicU32,
    done: AtomicBool,
}

#[test]
fn a_call_never_uses_an_address_space_freed_under_it() {
    // One guest whose address space is made and torn down over and over, as
    // a supervisor's guests come and go, and three copiers that each hold it
    // and check it stays alive while they do.
    let m = Arc::new(Machine {
        locks: (0..STRIPES).map(|_| Mutex::new(())).collect(),
        named: AtomicU32::new(0),
        alive: (0..SPACES).map(|_| AtomicBool::new(false)).collect(),
        used: AtomicU32::new(0),
        done: AtomicBool::new(false),
    });
    let copiers: Vec<_> = (0..3)
        .map(|_| {
            let m = Arc::clone(&m);
            thread::spawn(move || {
                let find = || Some(m.named.load(SeqCst)).filter(|&a| a != 0);
                while !m.done.load(SeqCst) {
                    let take = |s: usize| m.locks[s].lock().unwrap();
                    if let Some((asid, _held)) = hold(find, take) {
                        let alive = || m.alive[asid as usize].load(SeqCst);
                        assert!((0..8).all(|_| alive()), "{asid} freed under a call");
                        m.used.fetch_add(1, SeqCst);
                    }
                }
            })
        })
        .collect();
    for asid in 1..SPACES as u32 {
        m.alive[asid as usize].store(true, SeqCst);
        m.named.store(asid, SeqCst);
        // Alive until a copier has used it, or a while if none comes.
        let before = m.used.load(SeqCst);
        (0..100).take_while(|_| m.used.load(SeqCst) == before).for_each(|_| thread::yield_now());
        // The teardown: under the guest's lock, stop naming it, then free it.
        let _held = m.locks[stripe_of(asid)].lock().unwrap();
        m.named.store(0, SeqCst);
        m.alive[asid as usize].store(false, SeqCst);
    }
    m.done.store(true, SeqCst);
    copiers.into_iter().for_each(|c| c.join().unwrap());
    assert!(m.used.load(SeqCst) > 1000, "copiers seldom met a live address space");
}
