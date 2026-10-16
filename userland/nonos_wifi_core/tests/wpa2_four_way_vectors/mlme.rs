// NONOS Operating System (AGPL-3.0-or-later)

//! Through the MLME: one RSNE in the request and message 2, message 3 again.

use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};
use nonos_wifi_core::eapol::mic::{compute_mic, verify_mic};
use nonos_wifi_core::eapol::parse::parse;
use nonos_wifi_core::mlme::{Mlme, MlmeState};
use nonos_wifi_core::wpa::RSN_IE;

use super::vectors::*;

// A management frame from the access point to the station.
fn from_ap(subtype: u8, body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, subtype), SPA, AA, AA, 0).unwrap();
    f.extend_from_slice(body);
    f
}

// The RSN element in an association request, whole.
fn assoc_rsne(req: &[u8]) -> Vec<u8> {
    let mut off = MAC_HEADER_LEN + 4;
    while off + 2 <= req.len() {
        let end = off + 2 + req[off + 1] as usize;
        if req[off] == 48 {
            return req[off..end].to_vec();
        }
        off = end;
    }
    panic!("no RSNE in the association request");
}

#[test]
fn the_mlme_sends_the_same_rsne_in_the_association_request_and_message_2() {
    // Beacon of the transition network on SSID "IEEE": the station is joined
    // with PSK (a WPA2-only policy) and the passphrase "password".
    let mut beacon = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut beacon, frame_control(TYPE_MGMT, 8), [0xFF; 6], AA, AA, 0).unwrap();
    beacon.extend_from_slice(&[0u8; 8]);
    beacon.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
    beacon.extend_from_slice(&[0, 4, b'I', b'E', b'E', b'E']);
    beacon.extend_from_slice(&[1, 8, 0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24]);
    beacon.extend_from_slice(&[3, 1, 6]);
    beacon.extend_from_slice(&h(AP_RSNE_TRANSITION));
    beacon.extend_from_slice(&h(AP_RSNXE));

    let mut m = Mlme::new(SPA, b"IEEE", b"password", SNONCE);
    m.on_mgmt(&beacon).tx.expect("open authentication");
    let req = m.on_mgmt(&from_ap(11, &[0, 0, 2, 0, 0, 0])).tx.expect("association request");
    assert_eq!(assoc_rsne(&req), RSN_IE.to_vec(), "MFPC clear: PMF is not negotiated for PSK");
    m.on_mgmt(&from_ap(1, &[0x11, 0x04, 0, 0, 0x01, 0xC0]));
    assert_eq!(m.state(), MlmeState::FourWay);

    let m2 = m.on_eapol(&h(M1)).tx.expect("message 2");
    assert_eq!(m2, h(M2_TRANSITION));
    assert_eq!(&m2[99..], &assoc_rsne(&req)[..], "message 2 repeats the request's RSNE");
    let m4 = m.on_eapol(&h(M3_TRANSITION)).tx.expect("message 4");
    assert_eq!(m4, h(M4_TRANSITION));
    assert_eq!(m.state(), MlmeState::Connected);

    // Message 4 lost: the AP repeats message 3 with the next replay counter,
    // and the supplicant the data path took over answers it again.
    let mut sup = m.supplicant().expect("connected");
    let mut again = h(M3_TRANSITION);
    again[16] = 3;
    let kck = &h(PTK)[..16];
    let mic = compute_mic(kck, &again);
    again[81..97].copy_from_slice(&mic);
    let m4_again = sup.step(&again).reply.expect("message 4 again");
    assert_eq!(parse(&m4_again).unwrap().replay_counter, [0, 0, 0, 0, 0, 0, 0, 3]);
    assert!(verify_mic(kck, &m4_again));
}
