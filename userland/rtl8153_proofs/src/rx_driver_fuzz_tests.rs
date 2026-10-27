// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile transfers through the bound driver: every frame that comes up
//! is one the walk finds, whole, and nothing else does.

use nonos_usbnet::Nic;

use crate::helpers::up;
use crate::rx_fuzz_tests::{hostile, walk};

#[test]
fn hostile_transfers_come_up_through_the_driver_whole_or_not_at_all() {
    let (p, mut r) = up(0x6010);
    let mut seed = 0x2545_f491_4f6c_dd1d;
    for _ in 0..5_000 {
        let t = hostile(&mut seed);
        let want: Vec<_> = walk(&t).into_iter().filter(|(_, len)| *len <= 1514).collect();
        p.bus.0.borrow_mut().bulk_in.push_back(Ok(Some(t.clone())));
        let mut got = Vec::new();
        let mut out = [0u8; 1514];
        while let Some(n) = r.recv(&mut out).unwrap() {
            got.push(out[..n].to_vec());
        }
        assert_eq!(got.len(), want.len());
        for (frame, (start, len)) in got.iter().zip(want) {
            assert_eq!(frame[..], t[start..start + len]);
        }
    }
}
