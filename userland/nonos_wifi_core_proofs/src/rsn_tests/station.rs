// NONOS Operating System (AGPL-3.0-or-later)

//! The station's own RSNE, the RSNXE bit, and key data.

use nonos_wifi_core::eapol::kde::parse_key_data;
use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::rsnxe::advertises_h2e;
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy};
use nonos_wifi_core::wpa::RSN_IE;

use super::parse::body;

#[test]
fn the_station_rsne_for_wpa2_psk_is_the_shared_constant() {
    let psk = parse_rsne(&body(4, &[4], &[2], 0)).unwrap();
    let sel = select(&psk, false, JoinPolicy::ANY).unwrap();
    assert_eq!(station_rsne(&sel), RSN_IE, "the association request and message 2 agree");
    let sae = parse_rsne(&body(4, &[4], &[8], 0x00C0)).unwrap();
    let e = station_rsne(&select(&sae, true, JoinPolicy::ANY).unwrap());
    assert_eq!(e[19], 8, "AKM SAE");
    assert_eq!(u16::from_le_bytes([e[20], e[21]]), 0x00C0, "MFPC and MFPR");
    assert!(parse_rsne(&e[2..]).unwrap().akm_sae, "our own element parses back");
}

#[test]
fn the_rsnxe_h2e_bit_is_read() {
    assert!(advertises_h2e(&[0x20]));
    assert!(!advertises_h2e(&[0x00]));
    assert!(!advertises_h2e(&[]));
}

#[test]
fn key_data_is_walked_to_its_padding() {
    let mut kd = vec![48, 2, 1, 0]; // a minimal RSNE
    kd.extend_from_slice(&[244, 1, 0x20]); // RSNXE
    kd.extend_from_slice(&[0xDD, 22, 0x00, 0x0f, 0xac, 0x01, 0x06, 0x00]);
    kd.extend_from_slice(&[0x5A; 16]);
    kd.extend_from_slice(&[0xDD, 0, 0, 0]); // padding
    let p = parse_key_data(&kd).unwrap();
    assert_eq!(p.rsne, Some(&[48u8, 2, 1, 0][..]));
    assert_eq!(p.rsnxe, Some(&[244u8, 1, 0x20][..]));
    let g = p.gtk.unwrap();
    assert_eq!((g.key_id, g.key), (2, &[0x5Au8; 16][..]), "only the index bits of 0x06");
    assert!(p.igtk.is_none());
    // A KDE that runs past the end is refused; padding that does is not.
    assert!(parse_key_data(&[0xDD, 10, 0x00, 0x0f, 0xac, 0x01]).is_none());
    assert!(parse_key_data(&[0xDD, 0x00, 0x00]).is_some());
    assert!(parse_key_data(&[0xDD, 22, 0x00, 0x0f, 0xac, 0x01, 1]).is_none(), "a cut GTK KDE");
}
