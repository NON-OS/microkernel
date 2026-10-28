// NONOS Operating System (AGPL-3.0-or-later)

//! WPA3 through this driver on the model: the shared station's SAE
//! (hash-to-element) commit and confirm as authentication frames on ring 0,
//! association, the four-way handshake under the SAE PMK, and data sealed
//! under the new key, against a WPA3-Personal access point (PMF required)
//! and against a transition-mode one (PSK and SAE offered, SAE chosen).

use nonos_libc::set_net_core;

use crate::model::rig::{station, Station};
use crate::model::wire::{code, connect, netif, NET_CORE};
use crate::model::wpa3::{Wpa3, Wpa3Ap};
use crate::model::Shared;

fn on(ap: Box<dyn Wpa3Ap>) -> (Station, Shared) {
    let (st, c) = station();
    c.borrow_mut().wpa3 = Some(Wpa3::new(ap));
    set_net_core(NET_CORE);
    (st, c)
}

/// The authentication frames the chip sent: (algorithm, sequence).
fn auths(c: &Shared) -> Vec<(u16, u16)> {
    let le = |f: &[u8], at: usize| u16::from_le_bytes([f[at], f[at + 1]]);
    let sent = &c.borrow().sent;
    sent.iter()
        .filter(|t| t.frame[0] == 0xb0)
        .map(|t| (le(&t.frame, 24), le(&t.frame, 26)))
        .collect()
}

fn joins_and_seals(st: &mut Station, c: &Shared, ssid: &[u8], pass: &[u8]) {
    let r = connect(st, ssid, pass);
    assert_eq!(code(&r), 0, "joined: {r:?}");
    assert_eq!(auths(c), [(3, 1), (3, 2)], "SAE commit, then confirm, through ring 0");
    assert!(c.borrow().wpa3.as_ref().unwrap().ap.tk().is_some(), "message 4 keyed the AP");
    assert_eq!(netif(st, NET_CORE, 2, &[]).1, [1], "the link is up");
    let mut eth = crate::ap_sim::AA.to_vec();
    eth.extend_from_slice(&[0x02, 0, 0, 0, 0, 9, 0x08, 0x00]);
    eth.extend_from_slice(b"sealed under the SAE key");
    assert_eq!(netif(st, NET_CORE, 4, &eth).0, 0);
    let chip = c.borrow();
    let got = chip.wpa3.as_ref().unwrap().ap.got();
    assert!(got.iter().any(|b| b.ends_with(b"sealed under the SAE key")), "{got:?}");
}

#[test]
fn a_wpa3_personal_join_runs_sae_and_carries_data() {
    use crate::sae_air::{SaeAp, PASS, SSID};
    let (mut st, c) = on(Box::new(SaeAp::new(44)));
    joins_and_seals(&mut st, &c, SSID, PASS);
    assert_eq!(c.borrow().mcu.bss, Some(44), "the BSS's context holds 5 GHz channel 44");
}

#[test]
fn a_transition_mode_network_is_joined_by_sae() {
    use crate::sae_mixed::{SaeAp, PASS, SSID};
    let (mut st, c) = on(Box::new(SaeAp::new(6)));
    joins_and_seals(&mut st, &c, SSID, PASS);
}

#[test]
fn a_wrong_wpa3_password_never_keys_the_link() {
    use crate::sae_air::{SaeAp, SSID};
    let (mut st, c) = on(Box::new(SaeAp::new(1)));
    let r = connect(&mut st, SSID, b"wrong horse battery");
    assert_ne!(code(&r), 0);
    assert!(c.borrow().wpa3.as_ref().unwrap().ap.tk().is_none());
    let a = auths(&c);
    assert!(a.first() == Some(&(3, 1)) && a.iter().all(|x| x.0 == 3), "SAE ran: {a:?}");
}
