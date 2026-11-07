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

//! One thread: shape, order, batching, full, wrap, a corrupt peer, and the
//! doorbell's arm and ring. Each runs inside `loom::model`, whose atomics
//! the ring is built on here.

use crate::doorbell::{Armed, Bell};
use crate::ring::{Ring, ARMED, INDEX};
use crate::ring_error::RingError;
use crate::ring_header::RingHeader;
use crate::ring_sync::{AtomicU64, Ordering};
use std::cell::Cell;

fn words(n: usize) -> Vec<AtomicU64> {
    (0..n).map(|_| AtomicU64::new(0)).collect()
}

struct Count(Cell<u32>);

impl Bell for Count {
    fn ring(&self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn a_ring_refuses_a_shape_it_cannot_index() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(12);
        assert_eq!(Ring::<3>::new(&h, &s, 4).err(), None);
        assert_eq!(Ring::<3>::new(&h, &s, 3).err(), Some(RingError::Shape));
        assert_eq!(Ring::<2>::new(&h, &s, 4).err(), Some(RingError::Shape));
        assert_eq!(Ring::<12>::new(&h, &s, 1).err(), Some(RingError::Shape));
        assert_eq!(Ring::<0>::new(&h, &s[..0], 4).err(), Some(RingError::Shape));
    });
}

#[test]
fn records_arrive_whole_and_in_order_once_published() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(8);
        let ring = Ring::<2>::new(&h, &s, 4).unwrap();
        let mut p = ring.producer();
        let mut c = ring.consumer();
        p.push(&[1, 10]).unwrap();
        p.push(&[2, 20]).unwrap();
        assert_eq!(p.pending(), 2);
        assert_eq!(c.available(), Ok(0), "nothing is visible before publish");
        assert!(!p.publish());
        let mut got = Vec::new();
        assert_eq!(c.drain(|r| got.push(r)), Ok(2));
        assert_eq!(got, [[1, 10], [2, 20]]);
        assert_eq!(c.drain(|_| panic!("read twice")), Ok(0));
    });
}

#[test]
fn a_full_ring_refuses_until_the_consumer_releases() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(2);
        let ring = Ring::<1>::new(&h, &s, 2).unwrap();
        let mut p = ring.producer();
        let mut c = ring.consumer();
        p.push(&[1]).unwrap();
        p.push(&[2]).unwrap();
        assert_eq!(p.push(&[3]), Err(RingError::Full));
        assert!(!p.publish());
        assert_eq!(c.drain(|_| {}), Ok(2));
        p.push(&[3]).unwrap();
    });
}

#[test]
fn the_counters_wrap_without_losing_a_record() {
    loom::model(|| {
        let h = RingHeader::new();
        h.head.store(INDEX - 2, Ordering::Relaxed);
        h.tail.store(INDEX - 2, Ordering::Relaxed);
        let s = words(4);
        let ring = Ring::<1>::new(&h, &s, 4).unwrap();
        let mut p = ring.producer();
        let mut c = ring.consumer();
        let mut got = Vec::new();
        for round in 0..3u64 {
            for i in 0..4u64 {
                p.push(&[round * 10 + i]).unwrap();
            }
            assert!(!p.publish());
            c.drain(|r| got.push(r[0])).unwrap();
        }
        let want: Vec<u64> = (0..3u64).flat_map(|r| (0..4).map(move |i| r * 10 + i)).collect();
        assert_eq!(got, want);
    });
}

#[test]
fn a_head_past_the_capacity_is_corrupt_and_nothing_is_read() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(4);
        let ring = Ring::<1>::new(&h, &s, 4).unwrap();
        let mut c = ring.consumer();
        h.head.store(5, Ordering::Relaxed);
        assert_eq!(c.drain(|_| panic!("read from a corrupt ring")), Err(RingError::Corrupt));
        h.head.store(u32::MAX, Ordering::Relaxed);
        assert_eq!(c.available(), Err(RingError::Corrupt));
        assert_eq!(h.tail.load(Ordering::Relaxed), 0, "nothing released");
    });
}

#[test]
fn a_tail_ahead_of_the_head_is_corrupt_and_nothing_is_written() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(4);
        let ring = Ring::<1>::new(&h, &s, 4).unwrap();
        let mut p = ring.producer();
        h.tail.store(9, Ordering::Relaxed);
        assert_eq!(p.push(&[7]), Err(RingError::Corrupt));
        assert!(s.iter().all(|w| w.load(Ordering::Relaxed) == 0));
    });
}

#[test]
fn a_sleeping_consumer_sees_its_arm_in_the_head_word() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(4);
        let ring = Ring::<1>::new(&h, &s, 4).unwrap();
        let mut c = ring.consumer();
        assert_eq!(c.arm(), Ok(Armed::Sleep));
        assert_eq!(h.head.load(Ordering::Relaxed), ARMED);
        let mut p = ring.producer();
        p.push(&[3]).unwrap();
        assert!(p.publish(), "the publish reports the arm");
        assert_eq!(h.head.load(Ordering::Relaxed), 1, "and clears it");
    });
}

#[test]
fn a_peer_setting_bit_31_of_the_tail_moves_no_index() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(4);
        let ring = Ring::<1>::new(&h, &s, 4).unwrap();
        let mut p = ring.producer();
        h.tail.store(ARMED, Ordering::Relaxed);
        p.push(&[1]).unwrap();
        assert!(!p.publish());
        assert_eq!(ring.consumer().available(), Ok(1));
    });
}

#[test]
fn an_awake_consumer_costs_no_bell() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(4);
        let ring = Ring::<1>::new(&h, &s, 4).unwrap();
        let mut p = ring.producer();
        let bell = Count(Cell::new(0));
        for i in 0..3 {
            p.push(&[i]).unwrap();
            assert!(!p.publish_and_ring(&bell));
        }
        assert_eq!(bell.0.get(), 0);
    });
}

#[test]
fn an_armed_consumer_is_rung_once_and_disarmed() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(4);
        let ring = Ring::<1>::new(&h, &s, 4).unwrap();
        let mut p = ring.producer();
        let mut c = ring.consumer();
        let bell = Count(Cell::new(0));
        assert_eq!(c.arm(), Ok(Armed::Sleep));
        p.push(&[1]).unwrap();
        assert!(p.publish_and_ring(&bell));
        p.push(&[2]).unwrap();
        assert!(!p.publish_and_ring(&bell));
        assert_eq!(bell.0.get(), 1);
    });
}

#[test]
fn arming_with_records_waiting_says_read_them() {
    loom::model(|| {
        let h = RingHeader::new();
        let s = words(4);
        let ring = Ring::<1>::new(&h, &s, 4).unwrap();
        let mut p = ring.producer();
        let mut c = ring.consumer();
        p.push(&[1]).unwrap();
        assert!(!p.publish());
        assert_eq!(c.arm(), Ok(Armed::Ready));
        assert_eq!(h.head.load(Ordering::Relaxed) & ARMED, 0, "arm withdrawn");
    });
}
