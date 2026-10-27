// NONOS Operating System (AGPL-3.0-or-later)

//! The access point's side of a keyed link, for the proofs that send the
//! station what an access point would: data sealed under the pairwise key
//! or a group key at its index, heard by the chip, and what reaches the
//! stack of it.

use nonos_libc::mk_idle_ms;
use nonos_wifi_core::dot11::ccmp::encrypt;
use nonos_wifi_core::rsn::TemporalKey;

use super::rig::Station;
use super::wire::{netif, NET_CORE};
use super::Shared;
use crate::soft::data_frames::from_ds_of;

/// An LLC/SNAP header for IPv4, then `tag`.
pub fn ipv4(tag: &[u8]) -> Vec<u8> {
    let mut llc = vec![0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00];
    llc.extend_from_slice(tag);
    llc
}

/// The pairwise key the access point took at message 4.
pub fn ap_tk(c: &Shared) -> [u8; 16] {
    match c.borrow().air.as_ref().unwrap().ap.tk {
        Some(TemporalKey::Ccmp128(k)) => k,
        _ => panic!("the AP is not keyed for CCMP-128"),
    }
}

/// `llc` from the access point at `bssid` to `to`, sealed under `key` at
/// `pn` and key index `id`.
pub fn sealed(bssid: [u8; 6], to: [u8; 6], llc: &[u8], pn: u64, key: &[u8; 16], id: u8) -> Vec<u8> {
    let mut f = encrypt(&from_ds_of(bssid, to, llc), pn, key).unwrap();
    f[27] = 0x20 | id << 6;
    f
}

/// `llc` from the access point to the station, under the pairwise key.
pub fn unicast(c: &Shared, llc: &[u8], pn: u64) -> Vec<u8> {
    let (bssid, sta) = {
        let chip = c.borrow();
        let air = chip.air.as_ref().unwrap();
        (air.ap.bssid, air.sta)
    };
    sealed(bssid, sta, llc, pn, &ap_tk(c), 0)
}

/// `f` heard by the chip; the Ethernet frames that reached the stack.
pub fn hear(st: &mut Station, c: &Shared, f: Vec<u8>) -> Vec<Vec<u8>> {
    c.borrow_mut().receive(f);
    let mut got = Vec::new();
    for _ in 0..5 {
        mk_idle_ms(5);
        st.tick();
        while let (0, frame) = netif(st, NET_CORE, 5, &[]) {
            if frame.is_empty() {
                break;
            }
            got.push(frame);
        }
    }
    got
}
