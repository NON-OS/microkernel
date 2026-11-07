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

//! The shared-memory ring and doorbell of `nonos_surface`: what a record
//! costs to send and receive, and how many bells a stream rings. A record
//! is four words, the size of a present message (handle, buffer index,
//! damage rectangle packed in two words).

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Instant;

use nonos_surface::{Armed, Bell, Ring, RingError, RingHeader};

const W: usize = 4;
const CAP: u32 = 256;
const RECORDS: u64 = 2_000_000;

struct Shared {
    header: RingHeader,
    slots: Vec<AtomicU64>,
}

fn shared() -> Arc<Shared> {
    Arc::new(Shared {
        header: RingHeader::new(),
        slots: (0..CAP as usize * W).map(|_| AtomicU64::new(0)).collect(),
    })
}

struct Wake {
    rung: Mutex<u64>,
    cv: Condvar,
    count: AtomicU32,
}

#[derive(Clone)]
struct WakeRef(Arc<Wake>);

impl Bell for WakeRef {
    fn ring(&self) {
        self.0.count.fetch_add(1, Ordering::Relaxed);
        let mut n = self.0.rung.lock().unwrap_or_else(|e| e.into_inner());
        *n += 1;
        self.0.cv.notify_one();
    }
}

fn one_thread(batch: u64) -> f64 {
    let sh = shared();
    let Ok(ring) = Ring::<W>::new(&sh.header, &sh.slots, CAP) else { return f64::NAN };
    let mut p = ring.producer();
    let mut c = ring.consumer();
    let mut sum = 0u64;
    let start = Instant::now();
    let mut i = 0;
    while i < RECORDS {
        for _ in 0..batch {
            if p.push(&[i, i, i, i]).is_err() {
                break;
            }
            i += 1;
        }
        let _ = p.publish();
        let _ = c.drain(|r| sum = sum.wrapping_add(r[0]));
    }
    std::hint::black_box(sum);
    start.elapsed().as_nanos() as f64 / RECORDS as f64
}

fn two_threads(batch: u64) -> (f64, u32) {
    let sh = shared();
    let wake = Arc::new(Wake { rung: Mutex::new(0), cv: Condvar::new(), count: AtomicU32::new(0) });
    let (tx, bell) = (sh.clone(), WakeRef(wake.clone()));
    let start = Instant::now();
    let producer = thread::spawn(move || {
        let Ok(ring) = Ring::<W>::new(&tx.header, &tx.slots, CAP) else { return };
        let mut p = ring.producer();
        let mut i = 0;
        while i < RECORDS {
            let mut pushed = 0;
            while pushed < batch && i < RECORDS {
                match p.push(&[i, i, i, i]) {
                    Ok(()) => {
                        i += 1;
                        pushed += 1;
                    }
                    Err(RingError::Full) => break,
                    Err(_) => return,
                }
            }
            if !p.publish_and_ring(&bell) && pushed == 0 {
                thread::yield_now();
            }
        }
    });
    let Ok(ring) = Ring::<W>::new(&sh.header, &sh.slots, CAP) else { return (f64::NAN, 0) };
    let mut c = ring.consumer();
    let (mut got, mut seen) = (0u64, 0u64);
    while got < RECORDS {
        match c.drain(|_| {}) {
            Ok(0) => {}
            Ok(n) => {
                got += u64::from(n);
                continue;
            }
            Err(_) => break,
        }
        if let Ok(Armed::Sleep) = c.arm() {
            let mut n = wake.rung.lock().unwrap_or_else(|e| e.into_inner());
            while *n == seen {
                n = wake.cv.wait(n).unwrap_or_else(|e| e.into_inner());
            }
            seen = *n;
        }
    }
    let _ = producer.join();
    (start.elapsed().as_nanos() as f64 / RECORDS as f64, wake.count.load(Ordering::Relaxed))
}

pub fn run() {
    println!("# ring, {W}-word records, capacity {CAP}, {RECORDS} records");
    for batch in [1u64, 16] {
        let ns = one_thread(batch);
        println!("{:<44} {ns:>8.1} ns/record  host", format!("one thread, publish every {batch}"));
    }
    for batch in [1u64, 16] {
        let (ns, bells) = two_threads(batch);
        println!(
            "{:<44} {ns:>8.1} ns/record  bells={bells} ({:.4} per record)  host",
            format!("two threads, publish every {batch}"),
            f64::from(bells) / RECORDS as f64
        );
    }
}
