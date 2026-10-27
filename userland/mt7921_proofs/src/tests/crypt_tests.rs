// NONOS Operating System (AGPL-3.0-or-later)

//! CCMP-128 in the chip on the model, whose chip drops a protected frame
//! no installed key matches. A WPA2 join puts the pairwise key under the
//! access point's entry and the group key under the own; the station's
//! frames go out plaintext with `PROTECT_FRAME` and the chip seals them;
//! the access point's come up decrypted and pass the station's replay
//! check on the packet number the chip reports. A frame whose MIC fails
//! goes no further. When the firmware refuses the keys, nothing protected
//! from the access point reaches the station on such a chip.

use crate::ap_sim::GTK;
use crate::model::ap_tx::{ap_tk, hear, ipv4, unicast};
use crate::model::net::{joined, on_air, PASS, SSID};
use crate::model::wire::{code, connect, netif, NET_CORE};
use crate::model::Shared;

fn send(st: &mut crate::model::rig::Station, c: &Shared, tag: &[u8]) {
    let mut eth = vec![0xff; 6];
    eth.extend_from_slice(&[0x02, 0, 0, 0, 0, 9, 0x08, 0x00]);
    eth.extend_from_slice(tag);
    assert_eq!(netif(st, NET_CORE, 4, &eth).0, 0);
    assert!(c.borrow().air.as_ref().unwrap().ap.got.iter().any(|b| b.ends_with(tag)), "{tag:?}");
}

#[test]
fn a_wpa2_join_keys_the_chip_and_data_goes_both_ways_through_it() {
    let (mut st, c) = joined();
    let tk = ap_tk(&c);
    {
        let chip = c.borrow();
        assert_eq!(chip.keys.entries[&1].pairwise, Some(tk), "the AP's entry holds the TK");
        assert_eq!(chip.keys.entries[&19].group[1], Some(GTK), "the own entry, the GTK at 1");
    }
    send(&mut st, &c, b"sealed by the chip");
    let last = c.borrow().sent.last().cloned().unwrap();
    assert_eq!(last.txd[3] & 2, 2, "PROTECT_FRAME");
    assert_eq!(last.frame[1] & 0x40, 0x40, "Protected, as mac80211 hands it");
    assert_eq!(last.frame.len() + 16, last.air.unwrap().len(), "the chip added header and MIC");
    let got = hear(&mut st, &c, unicast(&c, &ipv4(b"to the station"), 7));
    assert!(got.iter().any(|f| f.ends_with(b"to the station")), "{got:?}");
    assert_eq!(c.borrow().keys.dropped, (0, 0));
}

#[test]
fn a_replay_or_a_forged_mic_never_reaches_the_stack() {
    let (mut st, c) = joined();
    let f = unicast(&c, &ipv4(b"once"), 40);
    assert_eq!(hear(&mut st, &c, f.clone()).len(), 1);
    assert!(hear(&mut st, &c, f).is_empty(), "the same packet number again");
    let mut forged = unicast(&c, &ipv4(b"forged"), 41);
    let n = forged.len();
    forged[n - 1] ^= 1;
    assert!(hear(&mut st, &c, forged).is_empty(), "ICV_ERR");
    assert_eq!(hear(&mut st, &c, unicast(&c, &ipv4(b"later"), 42)).len(), 1);
}

#[test]
fn keys_the_firmware_refuses_leave_the_station_deaf_on_a_dropping_chip() {
    let (mut st, c) = on_air();
    c.borrow_mut().faults.refuse_keys = true;
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    assert!(c.borrow().keys.entries.get(&1).is_none_or(|e| e.pairwise.is_none()));
    assert!(hear(&mut st, &c, unicast(&c, &ipv4(b"lost"), 3)).is_empty());
    assert!(c.borrow().keys.dropped.0 > 0, "the chip dropped it, as the risk says");
    c.borrow_mut().faults.pass_unkeyed = true;
    let got = hear(&mut st, &c, unicast(&c, &ipv4(b"opened by the station"), 4));
    assert_eq!(got.len(), 1, "a chip that passes it: the station's own CCMP opens it");
    send(&mut st, &c, b"sealed by the station");
    assert_eq!(c.borrow().sent.last().unwrap().txd[3] & 2, 0, "no PROTECT_FRAME then");
}
