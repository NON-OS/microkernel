// NONOS Operating System (AGPL-3.0-or-later)
//! The capture ring: reads in whole frames; a full ring loses its oldest
//! whole frames and says how many bytes.

use crate::ring::{Ring, RING_CAP};

#[test]
fn reads_are_whole_frames() {
    let mut r = Ring::new(4);
    r.put(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(r.take(7), (0, vec![1, 2, 3, 4]));
    assert_eq!(r.take(100), (0, vec![5, 6, 7, 8]), "the half frame waits");
}

#[test]
fn a_full_ring_drops_the_oldest_whole_frames() {
    let mut r = Ring::new(4);
    r.put(&vec![1u8; RING_CAP]);
    r.put(&[2, 2, 2, 2, 3, 3]);
    let (overrun, all) = r.take(usize::MAX);
    assert_eq!(overrun, 8, "two frames made room for six bytes");
    // The ones left and the 2s; the half frame of 3s waits for its rest.
    assert_eq!(all.len(), RING_CAP - 8 + 4);
    assert_eq!(&all[all.len() - 4..], &[2, 2, 2, 2]);
    assert_eq!(r.take(4).0, 0, "told once");
}

#[test]
fn a_put_larger_than_the_ring_keeps_its_newest_frames() {
    let mut r = Ring::new(2);
    let big: Vec<u8> = (0..RING_CAP + 6).map(|i| i as u8).collect();
    r.put(&big);
    let (overrun, all) = r.take(usize::MAX);
    assert_eq!((overrun, all.len()), (6, RING_CAP));
    assert_eq!(all.last(), big.last());
}

#[test]
fn a_packet_of_part_frames_keeps_its_whole_ones_and_stays_in_step() {
    let mut r = Ring::new(4);
    r.put_packet(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(r.take(100), (2, vec![1, 2, 3, 4]), "the part frame is lost, counted");
    r.put_packet(&[7, 8, 9, 10]);
    assert_eq!(r.take(100), (0, vec![7, 8, 9, 10]), "the next packet reads in step");
    r.put_packet(&[1, 2, 3]);
    assert_eq!(r.take(100), (3, vec![]));
}
