// NONOS Operating System (AGPL-3.0-or-later)
//! Bulk INs a hostile device could send, and frames of every length
//! handed down: a frame is taken whole or dropped, never cut, and nothing
//! panics.

use nonos_usbnet::found::fetch;
use nonos_usbnet::mock::drawn;
use nonos_usbnet::xhci::E_INVAL;
use nonos_usbnet::{Bind, Nic};

use crate::ecm::bind;
use crate::ecm::frame::received_len;
use crate::ecm::link::RX_LEN;
use crate::qemu::usb_net;

pub fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

#[test]
fn a_transfer_longer_than_a_frame_and_its_padding_byte_is_dropped_not_cut() {
    assert_eq!(received_len(60), Some(60));
    assert_eq!(received_len(1514), Some(1514));
    assert_eq!(received_len(1515), Some(1514), "the sender's padding byte");
    for n in [1516, 1518, 2047, 2048] {
        assert_eq!(received_len(n), None, "{n} bytes");
    }
}

#[test]
fn random_bulk_ins_come_up_whole_or_not_at_all() {
    let bus = usb_net(None);
    let found = fetch(&mut bus.clone(), 2).unwrap();
    let Bind::Ours(mut nic) = bind(bus.clone(), &found, drawn()) else { panic!("not bound") };
    let mut seed = 0x2545_F491_4F6C_DD1D;
    // A transfer that fills the receive buffer has not ended: it and the
    // transfer after it, its rest, are let go whatever they hold.
    let mut unended = false;
    for _ in 0..20_000 {
        let len = (next(&mut seed) % (RX_LEN as u64 + 1)) as usize;
        let xfer: Vec<u8> = (0..len).map(|_| next(&mut seed) as u8).collect();
        bus.0.borrow_mut().bulk_in.push_back(Ok(Some(xfer.clone())));
        let mut out = [0u8; 1514];
        let let_go = std::mem::replace(&mut unended, len == RX_LEN) || len == RX_LEN;
        match nic.recv(&mut out) {
            Ok(Some(n)) => {
                assert!(!let_go, "{len} bytes went up inside an unended transfer");
                assert!(n == len || (len == 1515 && n == 1514), "{len} came, {n} went up");
                assert_eq!(out[..n], xfer[..n]);
            }
            Ok(None) => assert!(let_go || !(14..=1515).contains(&len), "{len} bytes dropped"),
            Err(e) => panic!("{len} bytes: {e}"),
        }
    }
}

#[test]
fn frames_of_any_length_are_refused_or_sent_never_a_panic() {
    let bus = usb_net(None);
    let found = fetch(&mut bus.clone(), 2).unwrap();
    let Bind::Ours(mut nic) = bind(bus, &found, drawn()) else { panic!("not bound") };
    for len in [0, 13, 1515, 1600, 4096] {
        assert_eq!(nic.send(&vec![0u8; len]), Err(E_INVAL), "{len} bytes");
    }
    assert_eq!(nic.send(&[0u8; 1514]), Ok(()));
}
