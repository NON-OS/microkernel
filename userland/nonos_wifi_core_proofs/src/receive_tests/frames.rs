// NONOS Operating System (AGPL-3.0-or-later)

//! Frames from the access point.

use nonos_wifi_core::ccmp::ccm::ccm_encrypt;
use nonos_wifi_core::dot11::ccmp::{aad_nonce, view};

use super::vectors::h;
use crate::ap_sim::{AA, SPA};

pub(super) const STA: [u8; 6] = SPA;
pub(super) const AP: [u8; 6] = AA;
pub(super) const SRC: [u8; 6] = [0x02, 0, 0, 0, 0, 0x77];

// An AP-to-station frame: FromDS, QoS (TID 0) when `qos`, `body` from the
// LLC/SNAP header on, encrypted under `key` at `pn` with key index `kid`
// unless `key` is None.
pub(super) fn from_ap(
    dst: [u8; 6],
    body: &[u8],
    key: Option<&[u8; 16]>,
    pn: u64,
    kid: u8,
    qos: bool,
) -> Vec<u8> {
    let mut f = vec![if qos { 0x88 } else { 0x08 }, 0x02, 0, 0];
    f.extend_from_slice(&dst);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&SRC);
    f.extend_from_slice(&[0x10, 0x00]);
    if qos {
        f.extend_from_slice(&[0x00, 0x00]);
    }
    let Some(key) = key else {
        f.extend_from_slice(body);
        return f;
    };
    f[1] |= 0x40;
    let hdr_len = f.len();
    let p = pn.to_be_bytes();
    f.extend_from_slice(&[p[7], p[6], 0, 0x20 | (kid << 6), p[5], p[4], p[3], p[2]]);
    let v = view(&f).unwrap();
    let mut aad = [0u8; 32];
    let (nonce, n) = aad_nonce(&f, &v, pn, &mut aad);
    let mut ct = vec![0u8; body.len() + 8];
    ccm_encrypt(key, &nonce, &aad[..n], body, &mut ct).unwrap();
    assert_eq!(hdr_len, v.hdr_len);
    f.extend_from_slice(&ct);
    f
}

pub(super) fn ip_body(payload: &[u8]) -> Vec<u8> {
    let mut b = h("aaaa030000000800");
    b.extend_from_slice(payload);
    b
}

pub(super) fn eapol_body(eapol: &[u8]) -> Vec<u8> {
    let mut b = h("aaaa0300000088 8e");
    b.extend_from_slice(eapol);
    b
}
