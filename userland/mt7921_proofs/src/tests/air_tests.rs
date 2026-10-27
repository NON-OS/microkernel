// NONOS Operating System (AGPL-3.0-or-later)

//! The whole path on the model: the shared station over the driver's
//! radio, the radio over the modelled chip, the chip on the air with a
//! WPA2-PSK access point. The scan hears the network through the chip's
//! channel grants; the join authenticates, associates and runs the
//! four-way handshake through rings 0 and 2; data goes both ways sealed.

use nonos_libc::mk_idle_ms;

use crate::model::net::{joined, on_air, SSID};
use crate::model::wire::{netif, scan_names, NET_CORE};

#[test]
fn the_scan_hears_the_network_through_channel_grants() {
    let (mut st, c) = on_air();
    for _ in 0..400 {
        st.tick();
        if scan_names(&mut st).contains(&SSID.to_vec()) {
            assert!(c.borrow().mcu.grants > 0);
            return;
        }
    }
    panic!("the network was never heard");
}

#[test]
fn a_wpa2_join_associates_and_keys_the_link() {
    let (mut st, c) = joined();
    assert_eq!(netif(&mut st, NET_CORE, 2, &[]).1, [1], "the link is up");
    let chip = c.borrow();
    assert!(chip.air.as_ref().unwrap().ap.tk.is_some(), "the AP installed the pairwise key");
    assert_eq!(chip.mcu.bss, Some(6), "the BSS's context holds its channel");
    assert_eq!(chip.mcu.roc, None, "the join's channel request was given back");
}

#[test]
fn data_goes_out_sealed_and_comes_in_opened() {
    let (mut st, c) = joined();
    let mut eth = vec![0u8; 14];
    eth[0..6].copy_from_slice(&[0xff; 6]);
    (eth[12], eth[13]) = (0x08, 0x00);
    eth.extend_from_slice(b"hello over the modelled chip");
    assert_eq!(netif(&mut st, NET_CORE, 4, &eth).0, 0);
    let got = c.borrow().air.as_ref().unwrap().ap.got.clone();
    assert!(got.iter().any(|b| b.ends_with(b"hello over the modelled chip")), "{got:?}");
    c.borrow_mut().air.as_mut().unwrap().ap.chatter = true;
    for _ in 0..50 {
        mk_idle_ms(20);
        st.tick();
        let (status, frame) = netif(&mut st, NET_CORE, 5, &[]);
        if status == 0 && !frame.is_empty() {
            // The reply: the length, then the Ethernet frame.
            assert_eq!(&frame[16..18], &[0x08, 0x00], "an IPv4 frame came up");
            return;
        }
    }
    panic!("no frame came in");
}
