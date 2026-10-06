// NONOS Operating System (AGPL-3.0-or-later)
//! A bulk IN's frames queued and handed up one per receive; the pipe is
//! polled again only when the queue is empty. Random transfers never
//! panic and never yield a frame outside the packet area.

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Nic, Setup};

use crate::ax::rx::frames;
use crate::bound::bound;
use crate::chip::Chip;
use crate::layout::{frame, rx_transfer};

#[test]
fn three_frames_in_one_transfer_come_up_one_per_receive() {
    let (bus, mut nic) = bound(&Chip::default());
    let (a, b, c) = (frame(60, 1), frame(1514, 2), frame(342, 3));
    let good = rx_transfer(&[(&a, 0), (&b, 0), (&c, 0)], true);
    assert!(good.len() <= 4096);
    let bad = vec![0u8; 16];
    bus.0.borrow_mut().bulk_in.extend([Ok(Some(good)), Ok(Some(bad)), Err(-32)]);
    let mut out = [0u8; 1514];
    for want in [&a, &b, &c] {
        let n = nic.recv(&mut out).unwrap().unwrap();
        assert_eq!(&out[..n], &want[..]);
    }
    assert_eq!(bus.0.borrow().bulk_in.len(), 2, "one bulk IN for three frames");
    assert_eq!(nic.recv(&mut out), Ok(None), "a malformed transfer is dropped");
    assert_eq!(nic.recv(&mut out), Err(-32));
    let calls = bus.0.borrow().calls.clone();
    assert_eq!(
        calls[calls.len() - 2..],
        [Call::ResetBulk(true), Call::Out(Setup::new(2, 1, 0, 0x82), vec![])]
    );
    assert_eq!(nic.recv(&mut out), Ok(None));
}

#[test]
fn random_transfers_never_panic_or_point_outside_the_packets() {
    let mut seed = 0x2545_f491_4f6c_dd1du64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mut spans = Vec::new();
    for _ in 0..20_000 {
        let len = (next() % 4097) as usize;
        let mut x: Vec<u8> = (0..len).map(|_| next() as u8).collect();
        if len >= 4 && next() % 2 == 0 {
            // A plausible trailer: a small count and an offset inside.
            let (count, off) = (next() % 8, next() % len as u64);
            x[len - 4..].copy_from_slice(&((count | off << 16) as u32).to_le_bytes());
        }
        if frames(&x, &mut spans) && !spans.is_empty() {
            let off = (u32::from_le_bytes(x[len - 4..].try_into().unwrap()) >> 16) as usize;
            assert!(spans.iter().all(|&(at, l)| at + l <= off && (14..=1514).contains(&l)));
        }
    }
}
