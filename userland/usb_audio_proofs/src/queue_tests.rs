// NONOS Operating System (AGPL-3.0-or-later)
//! The PCM queue: whole writes, wrap-around, and silence when short.

use crate::serve::queue::{PcmQueue, QUEUE_BYTES};

fn bytes(from: u8, n: usize) -> Vec<u8> {
    (0..n).map(|i| from.wrapping_add(i as u8)).collect()
}

#[test]
fn a_write_is_taken_whole_or_not_at_all() {
    let mut q = PcmQueue::new();
    for _ in 0..QUEUE_BYTES / 4096 {
        assert!(q.push(&bytes(0, 4096)));
    }
    assert!(!q.fits(1));
    assert!(!q.push(&[1]));
    let mut out = vec![0u8; 1000];
    assert_eq!(q.pop_into(&mut out), 1000);
    assert!(!q.push(&bytes(0, 1001)));
    assert!(q.push(&bytes(0, 1000)));
}

#[test]
fn bytes_come_out_in_order_across_the_wrap() {
    let mut q = PcmQueue::new();
    let mut out = vec![0u8; QUEUE_BYTES - 100];
    assert!(q.push(&vec![7u8; QUEUE_BYTES - 100]));
    assert_eq!(q.pop_into(&mut out), QUEUE_BYTES - 100);
    let data = bytes(3, 4000);
    assert!(q.push(&data));
    let mut got = vec![0u8; 4000];
    assert_eq!(q.pop_into(&mut got), 4000);
    assert_eq!(got, data);
}

#[test]
fn a_short_queue_is_padded_with_silence() {
    let mut q = PcmQueue::new();
    assert!(q.push(&[9u8; 100]));
    let mut out = vec![0xaau8; 300];
    assert_eq!(q.pop_into(&mut out), 100);
    assert!(out[..100].iter().all(|&b| b == 9) && out[100..].iter().all(|&b| b == 0));
    assert_eq!(q.pop_into(&mut out), 0);
    assert!(out.iter().all(|&b| b == 0));
}

#[test]
fn clear_drops_what_was_queued() {
    let mut q = PcmQueue::new();
    assert!(q.push(&[5u8; 64]));
    q.clear();
    let mut out = [1u8; 64];
    assert_eq!(q.pop_into(&mut out), 0);
    assert!(q.fits(QUEUE_BYTES));
}
