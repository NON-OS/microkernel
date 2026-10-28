// NONOS Operating System (AGPL-3.0-or-later)

//! A WPA3-Personal join (SAE, protected management frames required) on the
//! model: the pairwise key, and the group key with the IGTK beside it, go
//! into the chip; data from the access point comes up through it; its
//! protected deauthentication, opened by the chip, ends the link where an
//! unprotected one does not; and the station's own, as it leaves, goes out
//! sealed by the chip under the pairwise key.

use nonos_libc::set_net_core;
use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt};

use crate::ap_sim::{AA, GTK};
use crate::model::ap_tx::{hear, ipv4, sealed};
use crate::model::rig::{station, Station};
use crate::model::wire::{code, connect, control, netif, NET_CORE};
use crate::model::wpa3::Wpa3;
use crate::model::Shared;
use crate::sae_air::{SaeAp, PASS, SSID};
use crate::soft::frames::mgmt_from;

fn wpa3() -> (Station, Shared, [u8; 16], [u8; 6]) {
    let (mut st, c) = station();
    c.borrow_mut().wpa3 = Some(Wpa3::new(Box::new(SaeAp::new(36))));
    set_net_core(NET_CORE);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    let tk = c.borrow().wpa3.as_ref().unwrap().ap.tk().unwrap();
    let sta = c.borrow().sent.last().unwrap().frame[10..16].try_into().unwrap();
    (st, c, tk, sta)
}

fn up(st: &mut Station) -> bool {
    netif(st, NET_CORE, 2, &[]).1 == [1]
}

#[test]
fn an_sae_join_keys_the_chip_with_the_igtk_and_data_comes_up() {
    let (mut st, c, tk, sta) = wpa3();
    {
        let chip = c.borrow();
        assert_eq!(chip.keys.entries[&1].pairwise, Some(tk));
        let own = &chip.keys.entries[&19];
        assert_eq!((own.group[1], own.igtk), (Some(GTK), Some([0x77; 16])), "GTK, IGTK");
    }
    let got = hear(&mut st, &c, sealed(AA, sta, &ipv4(b"over SAE"), 5, &tk, 0));
    assert!(got.iter().any(|e| e.ends_with(b"over SAE")), "{got:?}");
}

#[test]
fn only_a_protected_deauthentication_ends_the_link() {
    let (mut st, c, tk, sta) = wpa3();
    hear(&mut st, &c, mgmt_from(AA, 12, sta, &[7, 0]));
    assert!(up(&mut st), "an unprotected one is not the access point's word");
    hear(&mut st, &c, encrypt(&mgmt_from(AA, 12, sta, &[7, 0]), 6, &tk).unwrap());
    assert!(!up(&mut st), "the chip opened it and the station left");
}

#[test]
fn the_stations_own_deauthentication_is_sealed_by_the_chip() {
    let (mut st, c, tk, _) = wpa3();
    control(&mut st, 2, &[]);
    let chip = c.borrow();
    let deauth = chip.sent.iter().rev().find(|t| t.frame[0] == 0xc0).expect("a deauthentication");
    assert_eq!(deauth.txd[3] & 2, 2, "PROTECT_FRAME");
    let air = deauth.air.as_ref().unwrap();
    assert_eq!(decrypt(air, &tk).as_deref(), Some(&[3u8, 0][..]), "reason 3, under the TK");
}
