// NONOS Operating System (AGPL-3.0-or-later)

//! Through the MLME, as a person's join runs: one RSNE in the request and
//! message 2, and message 3 answered again.

use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};
use nonos_wifi_core::eapol::mic::{compute_mic, verify_mic};
use nonos_wifi_core::eapol::parse::parse;
use nonos_wifi_core::mlme::{Entropy, JoinRequest, Mlme, MlmeState, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;

use super::protected::{beacon, M2_PROTECTED, M3_PROTECTED, OWN_RSNE_PROTECTED};
use super::vectors::{h, AA, M1, PTK, SNONCE, SPA};

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
fn the_mlme_joins_a_psk_network_with_one_rsne_in_the_request_and_message_2() {
    // The PSK network requiring protection, joined as any network not saved
    // as WPA3 is, with the passphrase "password".
    let entropy = Entropy { snonce: SNONCE, sae: [0x22; SAE_ENTROPY] };
    let (ssid, passphrase) = (&b"IEEE"[..], &b"password"[..]);
    let req = JoinRequest { our_mac: SPA, ssid, passphrase, policy: JoinPolicy::ANY, entropy };
    let mut m = Mlme::join(&req);
    m.on_mgmt(&beacon()).tx.expect("open authentication");
    let req = m.on_mgmt(&from_ap(11, &[0, 0, 2, 0, 0, 0])).tx.expect("association request");
    assert_eq!(assoc_rsne(&req), h(OWN_RSNE_PROTECTED), "MFPC and MFPR, as required");
    m.on_mgmt(&from_ap(1, &[0x11, 0x04, 0, 0, 0x01, 0xC0]));
    assert_eq!(m.state(), MlmeState::FourWay);

    let m2 = m.on_eapol(&h(M1)).tx.expect("message 2");
    assert_eq!(m2, h(M2_PROTECTED));
    assert_eq!(&m2[99..], &assoc_rsne(&req)[..], "message 2 repeats the request's RSNE");
    let kck = &h(PTK)[..16];
    let m4 = m.on_eapol(&h(M3_PROTECTED)).tx.expect("message 4");
    assert_eq!((parse(&m4).unwrap().key_info, verify_mic(kck, &m4)), (0x030a, true));
    assert_eq!(m.state(), MlmeState::Connected);

    // Message 4 lost: the AP repeats message 3 with the next replay counter,
    // and the supplicant the data path took over answers it again.
    let mut sup = m.supplicant().expect("connected");
    let mut again = h(M3_PROTECTED);
    again[16] = 3;
    let mic = compute_mic(kck, &again);
    again[81..97].copy_from_slice(&mic);
    let m4_again = sup.step(&again).reply.expect("message 4 again");
    assert_eq!(parse(&m4_again).unwrap().replay_counter, [0, 0, 0, 0, 0, 0, 0, 3]);
    assert!(verify_mic(kck, &m4_again));
}
