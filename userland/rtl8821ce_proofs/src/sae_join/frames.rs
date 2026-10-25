// NONOS Operating System (AGPL-3.0-or-later)

//! The frames the WPA3 access point of the join proofs sends: its beacon, its
//! SAE authentication frames, its association response and its EAPOL.

use crate::ap_sim::AP_RSNE_MIXED;

pub const STA: [u8; 6] = [0x02, 0, 0, 0, 0, 0x02];
pub const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];
pub const SSID: &[u8] = b"home";
pub const PASS: &[u8] = b"correct horse";
pub const SNAP_EAPOL: [u8; 8] = [0xAA, 0xAA, 0x03, 0, 0, 0, 0x88, 0x8E];

/// The beacon: WPA2 and WPA3 offered, MFP capable, hash to element.
pub fn beacon() -> Vec<u8> {
    let mut b = mgmt(0x80, [0xFF; 6], &[0; 8]);
    b.extend_from_slice(&[0x64, 0, 0x11, 0x04, 0, SSID.len() as u8]);
    b.extend_from_slice(SSID);
    b.extend_from_slice(&[3, 1, 6]);
    b.extend_from_slice(&AP_RSNE_MIXED);
    b.extend_from_slice(&[244, 1, 0x20]);
    b
}

pub fn mgmt(fc0: u8, dst: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![fc0, 0, 0, 0];
    [dst, AP, AP].iter().for_each(|a| f.extend_from_slice(a));
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(body);
    f
}

pub fn sae_auth(seq: u16, status: u16, body: &[u8]) -> Vec<u8> {
    let head = [3u16, seq, status].map(u16::to_le_bytes).concat();
    mgmt(0xB0, STA, &[head, body.to_vec()].concat())
}

// An EAPOL frame from the AP to the station: FromDS, in the clear.
pub fn eapol(m: &[u8]) -> Vec<u8> {
    let mut f = vec![0x08, 0x02, 0, 0];
    [STA, AP, AP].iter().for_each(|a| f.extend_from_slice(a));
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&SNAP_EAPOL);
    f.extend_from_slice(m);
    f
}
