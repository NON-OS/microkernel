// NONOS Operating System (AGPL-3.0-or-later)
//! Splitting a bulk IN into frames, as rndis_rx_fixup does, over messages
//! laid out as QEMU's usbnet_receive builds them and over hostile ones.

use std::collections::VecDeque;
use std::ops::Range;

use crate::device::{packet, words};
use crate::rndis::batch::frames;

fn split(t: &[u8]) -> (Vec<Vec<u8>>, u32) {
    let mut q: VecDeque<Range<usize>> = VecDeque::new();
    let dropped = frames(t, &mut q);
    (q.into_iter().map(|r| t[r].to_vec()).collect(), dropped)
}

#[test]
fn one_qemu_message_and_a_batch_with_trailing_padding() {
    let (a, b) = ([0x11u8; 60], [0x22u8; 1514]);
    assert_eq!(split(&packet(&a)), (vec![a.to_vec()], 0));
    let mut t = [packet(&a), packet(&b), packet(&a)].concat();
    t.resize(t.len().next_multiple_of(512), 0);
    assert_eq!(split(&t), (vec![a.to_vec(), b.to_vec(), a.to_vec()], 0));
}

#[test]
fn a_bad_envelope_ends_the_walk_and_a_bad_frame_skips_one_message() {
    let a = [0x33u8; 64];
    let mut wrong_type = packet(&a);
    wrong_type[0] = 2;
    assert_eq!(split(&[packet(&a), wrong_type, packet(&a)].concat()), (vec![a.to_vec()], 1));
    for len in [0u32, 8, 43, 109, u32::MAX] {
        let mut m = packet(&a);
        m[4..8].copy_from_slice(&len.to_le_bytes());
        let (got, dropped) = split(&[packet(&a), m].concat());
        assert_eq!((got.len(), dropped), (1, 1), "MessageLength {len}");
    }
    for (at, v) in [(8, u32::MAX), (8, 35), (8, 37), (12, u32::MAX), (12, 13), (12, 1515), (12, 65)]
    {
        let mut m = packet(&a);
        m[at..at + 4].copy_from_slice(&v.to_le_bytes());
        let (got, dropped) = split(&[m, packet(&a)].concat());
        assert_eq!((got.len(), dropped), (1, 1), "word {at} = {v:#x}");
    }
}

#[test]
fn per_packet_info_before_the_data_is_stepped_over() {
    let mut m = words(&[1, 44 + 8 + 20, 36 + 8, 20, 0, 0, 0, 36, 8, 0, 0, 0xAA, 0xBB]);
    m.extend_from_slice(&[0x44; 20]);
    assert_eq!(split(&m), (vec![vec![0x44; 20]], 0));
}
