// NONOS Operating System (AGPL-3.0-or-later)
//! Random bulk IN transfers, and QEMU messages with random bytes changed:
//! every frame the walk hands up lies inside the transfer and is an
//! Ethernet frame, and nothing panics.

use std::collections::VecDeque;

use crate::device::packet;
use crate::rndis::batch::frames;

#[test]
fn random_and_mutated_transfers_never_leave_the_transfer() {
    let mut seed = 0x2545_F491_4F6C_DD1Du64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for round in 0..20_000 {
        let mut t = [packet(&[0x55; 100]), packet(&[0x66; 300])].concat();
        if round % 2 == 0 {
            t = (0..(next() % 700) as usize).map(|_| next() as u8).collect();
        }
        for _ in 0..(next() % 6) {
            let at = (next() as usize) % t.len().max(1);
            if let Some(b) = t.get_mut(at) {
                *b = next() as u8;
            }
        }
        let mut q = VecDeque::new();
        frames(&t, &mut q);
        assert!(q.iter().all(|r| r.end <= t.len() && (14..=1514).contains(&r.len())));
    }
}
