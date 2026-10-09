// NONOS Operating System (AGPL-3.0-or-later)
// The driver's own `idle`, on a ring in host memory: a request is out while
// the device's used index trails the avail index, across the u16 wrap. A
// completion counted for the wrong request handed back the last request's
// sectors, and the model load failed its seal check on the next boot.

use super::Queue;
use crate::constants::VQ_REGION_SIZE;

const SIZE: u16 = 128;

struct Ring {
    region: Vec<u64>,
    header: Vec<u64>,
    data: Vec<u64>,
}

impl Ring {
    fn new() -> Self {
        Ring { region: vec![0; VQ_REGION_SIZE / 8], header: vec![0; 512], data: vec![0; 512] }
    }

    fn queue(&mut self) -> Queue {
        let region = self.region.as_mut_ptr() as u64;
        let header = self.header.as_mut_ptr() as u64;
        let data = self.data.as_mut_ptr() as u64;
        Queue::new(region, 0, SIZE, header, 0, data, 0)
    }
}

fn set(at: *mut u8, value: u16) {
    // SAFETY: `at` is inside the ring's own region, two aligned bytes.
    unsafe { core::ptr::write_volatile(at.cast::<u16>(), value) }
}

fn posted(q: &Queue, n: u16) {
    // SAFETY: the avail index sits two bytes into the avail ring.
    set(unsafe { q.region_va.add(q.avail_offset + 2) }, n);
}

fn answered(q: &Queue, n: u16) {
    // SAFETY: the used index sits two bytes into the used ring.
    set(unsafe { q.region_va.add(q.used_offset + 2) }, n);
}

#[test]
fn a_fresh_queue_is_idle() {
    let mut ring = Ring::new();
    let q = ring.queue();
    assert!(q.idle());
}

#[test]
fn a_request_given_up_on_keeps_the_queue_busy_until_it_is_answered() {
    let mut ring = Ring::new();
    let q = ring.queue();
    posted(&q, 1);
    assert!(!q.idle(), "the request is the device's");
    posted(&q, 2);
    answered(&q, 1);
    assert!(!q.idle(), "the first answer is not the second request's");
    answered(&q, 2);
    assert!(q.idle());
}

#[test]
fn the_indices_are_compared_across_their_wrap() {
    let mut ring = Ring::new();
    let q = ring.queue();
    posted(&q, 0);
    answered(&q, u16::MAX);
    assert!(!q.idle(), "one request past the wrap is still out");
    answered(&q, 0);
    assert!(q.idle());
}
