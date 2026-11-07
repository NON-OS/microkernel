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

//! Two threads, every interleaving loom's model allows: records cross
//! whole and in order through a ring smaller than the stream, and a
//! consumer that sleeps on the doorbell, while a producer waits on a full
//! ring, is always woken (a lost wake leaves
//! it blocked, which loom reports as a deadlock).

use crate::doorbell::{Armed, Bell};
use crate::ring::Ring;
use crate::ring_error::RingError;
use crate::ring_header::RingHeader;
use crate::ring_sync::AtomicU64;
use loom::sync::{Arc, Condvar, Mutex};
use loom::thread;

struct Shared {
    header: RingHeader,
    slots: Vec<AtomicU64>,
}

fn shared(capacity: usize, w: usize) -> Arc<Shared> {
    Arc::new(Shared { header: RingHeader::new(), slots: (0..capacity * w).map(|_| AtomicU64::new(0)).collect() })
}

struct Rung {
    count: Mutex<u32>,
    cv: Condvar,
}

impl Bell for Arc<Rung> {
    fn ring(&self) {
        let mut n = self.count.lock().unwrap();
        *n += 1;
        self.cv.notify_one();
    }
}

#[test]
fn a_stream_longer_than_the_ring_arrives_whole_and_in_order() {
    loom::model(|| {
        let sh = shared(2, 2);
        let tx = sh.clone();
        let producer = thread::spawn(move || {
            let ring = Ring::<2>::new(&tx.header, &tx.slots, 2).unwrap();
            let mut p = ring.producer();
            let mut i = 0u64;
            while i < 3 {
                match p.push(&[i, !i]) {
                    Ok(()) => {
                        assert!(!p.publish());
                        i += 1;
                    }
                    Err(RingError::Full) => thread::yield_now(),
                    Err(e) => panic!("{e:?}"),
                }
            }
        });
        let ring = Ring::<2>::new(&sh.header, &sh.slots, 2).unwrap();
        let mut c = ring.consumer();
        let mut got = Vec::new();
        while got.len() < 3 {
            if c.drain(|r| got.push(r)).unwrap() == 0 {
                thread::yield_now();
            }
        }
        producer.join().unwrap();
        assert_eq!(got, [[0, !0], [1, !1], [2, !2]]);
    });
}

#[test]
fn a_sleeping_consumer_is_never_left_asleep() {
    loom::model(|| {
        let sh = shared(2, 1);
        let bell = Arc::new(Rung { count: Mutex::new(0), cv: Condvar::new() });
        let (tx, tx_bell) = (sh.clone(), bell.clone());
        let producer = thread::spawn(move || {
            let ring = Ring::<1>::new(&tx.header, &tx.slots, 2).unwrap();
            let mut p = ring.producer();
            let mut i = 0u64;
            while i < 3 {
                match p.push(&[i]) {
                    Ok(()) => {
                        p.publish_and_ring(&tx_bell);
                        i += 1;
                    }
                    Err(RingError::Full) => thread::yield_now(),
                    Err(e) => panic!("{e:?}"),
                }
            }
        });
        let ring = Ring::<1>::new(&sh.header, &sh.slots, 2).unwrap();
        let mut c = ring.consumer();
        let mut got = Vec::new();
        let mut seen = 0u32;
        while got.len() < 3 {
            if c.drain(|r| got.push(r[0])).unwrap() > 0 {
                continue;
            }
            if c.arm().unwrap() == Armed::Sleep {
                let mut n = bell.count.lock().unwrap();
                while *n == seen {
                    n = bell.cv.wait(n).unwrap();
                }
                seen = *n;
            }
        }
        producer.join().unwrap();
        assert_eq!(got, [0, 1, 2]);
    });
}
