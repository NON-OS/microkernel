// NONOS Operating System (AGPL-3.0-or-later)
//! The bytes a client is to read, over seeded runs: a read from the
//! device only while a whole page fits, as pump.rs reads, and the client's
//! takes of any size. Every byte the device sent is read, once, in order;
//! a put past the room (no read makes one, the lie here is the caller's)
//! keeps what fits and no more.

use std::collections::VecDeque;

use crate::acm::Rx;
use crate::notice_sweep::rng::{rounds, Rng};

const PAGE: usize = 4096;
const CAP: usize = 16 * 1024;

#[test]
fn every_byte_sent_while_it_fits_is_read_once_and_in_order() {
    let (mut overfilled, mut full) = (0, 0);
    for seed in 1..rounds(5_000) {
        let r = &mut Rng::new(seed);
        let (mut rx, mut q, mut next) = (Rx::default(), VecDeque::new(), 0u8);
        for step in 0..r.range(1, 60) {
            let at = format!("seed {seed}, step {step}");
            assert_eq!(rx.fits(PAGE), CAP - q.len() >= PAGE, "{at}: fits");
            assert_eq!(rx.queued(), q.len(), "{at}: queued");
            match r.range(0, 9) {
                0..=4 if rx.fits(PAGE) => {
                    let n = [PAGE, 0, 1, 64, r.range(0, PAGE)][r.range(0, 4)];
                    let b: Vec<u8> = (0..n)
                        .map(|_| {
                            next = next.wrapping_add(1);
                            next
                        })
                        .collect();
                    rx.put(&b);
                    q.extend(b);
                }
                0..=4 => full += 1,
                5 => {
                    let n = r.range(1, 2 * CAP);
                    let b: Vec<u8> = (0..n).map(|i| i as u8).collect();
                    let keep = n.min(CAP - q.len());
                    overfilled += (keep < n) as u32;
                    rx.put(&b);
                    q.extend(&b[..keep]);
                    next = 0;
                }
                _ => {
                    let most = [usize::MAX, 0, 1, r.range(0, 3 * PAGE)][r.range(0, 3)];
                    let want: Vec<u8> = q.drain(..most.min(q.len())).collect();
                    assert_eq!(rx.take(most), want, "{at}: take");
                }
            }
        }
        assert_eq!(rx.take(usize::MAX), q.drain(..).collect::<Vec<_>>(), "seed {seed}: the rest");
    }
    assert!(cfg!(miri) || (overfilled >= 500 && full >= 500), "{overfilled} {full}");
}
