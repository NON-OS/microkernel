// NONOS Operating System (AGPL-3.0-or-later)

//! The scenes the end-to-end proofs start from: a station on a chip whose
//! air holds a WPA2-PSK access point on channel 6, before and after its
//! join; and the association the station tells its radio.

use nonos_libc::set_net_core;
use nonos_wifi_core::rsn::Cipher;
use nonos_wifi_core::softmac::Association;

use super::air::Air;
use super::rig::{station, Station};
use super::wire::{code, connect, NET_CORE};
use super::Shared;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;

pub const SSID: &[u8] = b"ModelNet";
pub const PASS: &[u8] = b"correct horse battery";
pub const AP: [u8; 6] = [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

pub fn on_air() -> (Station, Shared) {
    let (st, c) = station();
    c.borrow_mut().air = Some(Air::new(SimAp::new(SSID, PASS, 6, &RSNE_PSK)));
    set_net_core(NET_CORE);
    (st, c)
}

pub fn joined() -> (Station, Shared) {
    let (mut st, c) = on_air();
    let r = connect(&mut st, SSID, PASS);
    assert_eq!(code(&r), 0, "the join failed: {r:?}");
    (st, c)
}

/// An association with `AP`: aid 5, four rates, CCMP-128.
pub fn assoc() -> Association {
    let mut rates = [0u8; 16];
    rates[..4].copy_from_slice(&[0x82, 0x84, 0x0c, 0x98]);
    Association {
        bssid: AP,
        aid: 5,
        capability: 0x411,
        beacon_int: 100,
        rates,
        n_rates: 4,
        cipher: Cipher::Ccmp128,
    }
}
