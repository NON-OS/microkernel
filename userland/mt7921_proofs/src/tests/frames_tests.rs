// NONOS Operating System (AGPL-3.0-or-later)

//! Frames through the radio on the model: a frame to the AP under its
//! record's entry, any other under the interface's own; management at the
//! band's lowest rate; every token back from the chip's reports; a frame
//! heard comes up whole with its signal, never marked decrypted.

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

use crate::model::rig::up;
use crate::model::rxd::rx_buffer;

const AP: [u8; 6] = [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

fn mgmt(a1: [u8; 6], len: usize) -> Vec<u8> {
    let mut f = vec![0u8; len];
    f[0] = 0xb0;
    f[4..10].copy_from_slice(&a1);
    f
}

#[test]
fn a_frame_to_the_ap_goes_under_its_entry_and_others_under_the_interfaces() {
    let (mut r, c) = up();
    r.tune(Band::Ghz5, 36);
    r.set_bss(Some(AP));
    assert!(r.send(&mgmt(AP, 30)));
    assert!(r.send(&mgmt([0xff; 6], 30)));
    let sent = c.borrow().sent.clone();
    assert_eq!((sent[0].wcid(), sent[1].wcid()), (1, 19));
    assert_eq!((sent[0].rate(), sent[0].frame.clone()), (0x4b, mgmt(AP, 30)), "6 Mb/s on 5 GHz");
}

#[test]
fn every_token_comes_back_and_a_silent_chip_still_ages_them_out() {
    let (mut r, c) = up();
    r.tune(Band::Ghz2, 1);
    let mut buf = [0u8; 2400];
    for _ in 0..200 {
        assert!(r.send(&mgmt([0xff; 6], 40)), "200 frames over 64 tokens");
        let _ = r.recv(&mut buf);
    }
    c.borrow_mut().faults.no_txfree = true;
    let sent = (0..70).filter(|_| r.send(&mgmt([0xff; 6], 40))).count();
    assert!(sent < 70, "with no reports the tokens run out");
    nonos_libc::mk_idle_ms(crate::mac::tokens::STALE_MS + 1);
    assert!(r.send(&mgmt([0xff; 6], 40)), "until they go stale");
    assert!(!r.send(&[0u8; 2049]) && !r.send(&[0u8; 10]), "longer than a buffer, or no header");
}

#[test]
fn a_frame_heard_comes_up_whole_with_its_signal() {
    let (mut r, c) = up();
    r.tune(Band::Ghz2, 1);
    let f: Vec<u8> = (0..60).collect();
    c.borrow_mut().push_rx(2, rx_buffer(&f, -47));
    let mut out = [0u8; 2400];
    let got = r.recv(&mut out).unwrap();
    assert_eq!((got.len, got.rssi_dbm, got.decrypted), (60, -47, false));
    assert_eq!(&out[..60], &f[..]);
    assert!(r.recv(&mut out).is_none());
}
