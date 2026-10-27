// NONOS Operating System (AGPL-3.0-or-later)

//! The chip's keys over a link's life on the model: a group rekey's key
//! goes in at its index, its message 2 out sealed by the chip, and group
//! data under it comes up; leaving takes every key out of the chip; and no
//! key is left in DMA memory or named on the serial line.

use nonos_libc::{dma_holds, take_lines};
use nonos_wifi_core::wpa::akm::Akm;

use crate::ap_sim::{group1, gtk_kde, GTK};
use crate::model::ap_tx::{ap_tk, hear, ipv4, sealed, unicast};
use crate::model::net::joined;
use crate::model::wire::{control, netif, NET_CORE};
use crate::soft::data_frames::eapol_from;

const NEW_GTK: [u8; 16] = [0x5a; 16];

#[test]
fn a_group_rekey_puts_the_new_key_in_the_chip_and_group_data_comes_up() {
    let (mut st, c) = joined();
    let (bssid, sta, ptk) = {
        let chip = c.borrow();
        let air = chip.air.as_ref().unwrap();
        (air.ap.bssid, air.sta, air.ap.ptk.clone().unwrap().1)
    };
    let m1 = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &NEW_GTK));
    let llc = eapol_from(bssid, sta, &m1)[24..].to_vec();
    hear(&mut st, &c, unicast(&c, &llc, 9));
    let chip = c.borrow();
    assert_eq!(chip.keys.entries[&19].group[2], Some(NEW_GTK), "the new key, at index 2");
    let m2 = chip.sent.last().unwrap();
    assert_eq!(m2.txd[3] & 2, 2, "message 2 went for the chip to seal");
    let got = &chip.air.as_ref().unwrap().ap.got;
    assert!(got.last().unwrap().starts_with(&[0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e]), "{got:?}");
    drop(chip);
    let f = sealed(bssid, [0xff; 6], &ipv4(b"to the group"), 1, &NEW_GTK, 2);
    let up = hear(&mut st, &c, f);
    assert!(up.iter().any(|e| e.ends_with(b"to the group")), "{up:?}");
}

#[test]
fn leaving_takes_every_key_out_of_the_chip() {
    let (mut st, c) = joined();
    assert!(c.borrow().keys.entries[&1].pairwise.is_some());
    control(&mut st, 2, &[]);
    let chip = c.borrow();
    for (wcid, e) in &chip.keys.entries {
        assert!(e.pairwise.is_none() && e.group.iter().all(Option::is_none), "entry {wcid}");
    }
    assert_eq!(netif(&mut st, NET_CORE, 2, &[]).1, [0], "the link is down");
}

#[test]
fn no_key_is_left_in_dma_memory_or_named_on_the_line() {
    let _ = take_lines();
    let (_st, c) = joined();
    let tk = ap_tk(&c);
    assert!(c.borrow().keys.installs >= 2, "the pairwise and the group key went in");
    assert!(!dma_holds(&tk) && !dma_holds(&GTK), "a command's copy outlived its answer");
    let lines = take_lines();
    assert!(lines.iter().any(|l| l.contains("pairwise key in the chip")), "{lines:?}");
    let hex: String = tk.iter().map(|b| format!("{b:02x}")).collect();
    for l in &lines {
        assert!(!l.contains(&hex) && !l.contains(&hex.to_uppercase()), "{l:?}");
    }
}
