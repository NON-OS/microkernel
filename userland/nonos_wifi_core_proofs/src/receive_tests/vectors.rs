// NONOS Operating System (AGPL-3.0-or-later)

//! CCMP against the annex M.6.4 vector and a QoS frame.

use nonos_wifi_core::dot11::ccmp::{decrypt, view};
use nonos_wifi_core::dot11::data::protect;

pub(super) fn h(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

const TK_M64: &str = "c97c1f67ce371185514a8a19f2bdd52f";

const M64_PLAIN: &str = "0848c32c0fd2e128a57c5030f1844408abaea5b8fcba8033
    f8ba1a55d02f85ae967bb62fb6cda8eb7e78a050";

const M64_CIPHER: &str = "0848c32c0fd2e128a57c5030f1844408abaea5b8fcba8033
    0ce7002076970 3b5 f3d0a2fe9a3dbf2342a643e43246e80c3c04d0197845ce0b16f97623";

const QOS_CIPHER: &str = "88420000020000000002020000000001020000000077301225000201002000000000
    be61194719d3a6a5a42a2593b793f93ec4bb74166420a43f7a";

pub(super) fn tk() -> [u8; 16] {
    h(TK_M64).try_into().unwrap()
}

#[test]
fn ccmp_matches_the_annex_m64_vector() {
    let enc = protect(&h(M64_PLAIN), 0xB5039776E70C, &tk()).unwrap();
    assert_eq!(enc, h(M64_CIPHER), "encrypted MPDU");
    assert_eq!(decrypt(&enc, &tk()).unwrap(), h(M64_PLAIN)[24..].to_vec(), "decrypts back");
}

#[test]
fn a_qos_frame_decrypts_with_its_tid_in_nonce_and_aad() {
    let f = h(QOS_CIPHER);
    let v = view(&f).unwrap();
    assert_eq!((v.hdr_len, v.tid), (26, 5));
    let plain = decrypt(&f, &tk()).expect("the QoS frame decrypts");
    assert_eq!(&plain[..8], &h("aaaa030000000800")[..]);
    assert_eq!(&plain[8..], b"hello qos");
    let mut flipped = f.clone();
    flipped[24] ^= 0x01; // another TID: a different nonce and AAD
    assert!(decrypt(&flipped, &tk()).is_none(), "the TID is authenticated");
}
