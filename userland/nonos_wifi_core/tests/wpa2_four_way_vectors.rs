// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! WPA2-PSK (AKM 00-0F-AC:2, CCMP, key descriptor version 2) against known
//! answers that do not come from this crate. PBKDF2: IEEE Std 802.11-2020
//! J.4.2, as hostap's crypto module tests carry it. PRF-SHA1: the J.3.2
//! "prefix" vectors from the same tests. The four-way handshake: messages 1
//! and 3 as an authenticator frames them, and the messages 2 and 4 expected
//! back, byte for byte, built by an independent reference (Python hashlib and
//! the `cryptography` RFC 3394 key wrap, following hostap's
//! tests/hwsim/test_ap_psk.py `pmk_to_ptk`, `build_eapol` and
//! `eapol_key_mic`). The access point is a WPA2/WPA3 transition network (AKM
//! 2 and 8, MFPC=1, MFPR=0, an RSNXE) joined with PSK, and a PSK network that
//! requires management frame protection, whose message 3 adds the IGTK KDE.
//! Through the MLME, the association request's RSNE is the one message 2
//! repeats, and a message 3 repeated after a lost message 4 is answered.

use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};
use nonos_wifi_core::eapol::mic::{compute_mic, verify_mic};
use nonos_wifi_core::eapol::parse::parse;
use nonos_wifi_core::mlme::{Mlme, MlmeState};
use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy, Pmf};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::pbkdf2::pbkdf2_sha1;
use nonos_wifi_core::wpa::prf::prf;
use nonos_wifi_core::wpa::ptk::{pmk, ptk};
use nonos_wifi_core::wpa::supplicant::{Config, State, Supplicant};
use nonos_wifi_core::wpa::RSN_IE;

fn h(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn pbkdf2_psk_matches_annex_j4() {
    let cases: [(&[u8], &[u8], &str); 3] = [
        (b"password", b"IEEE", "f42c6fc52df0ebef9ebb4b90b38a5f902e83fe1b135a70e23aed762e9710a12e"),
        (
            b"ThisIsAPassword",
            b"ThisIsASSID",
            "0dc0d6eb90555ed6419756b9a15ec3e3209b63df707dd508d14581f8982721af",
        ),
        (
            b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            b"ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ",
            "becb93866bb8c3832cb777c2f559807c8c59afcb6eae734885001300a981cc62",
        ),
    ];
    for (pass, ssid, want) in cases {
        assert_eq!(pmk(pass, ssid).to_vec(), h(want), "{}", String::from_utf8_lossy(pass));
        let mut out = [0u8; 32];
        pbkdf2_sha1(pass, ssid, 4096, &mut out);
        assert_eq!(out.to_vec(), h(want));
    }
}

#[test]
fn prf_sha1_matches_annex_j3() {
    let cases: [(Vec<u8>, Vec<u8>, &str); 3] = [
        (
            vec![0x0b; 20],
            b"Hi There".to_vec(),
            "bcd4c650b30b9684951829e0d75f9d54b862175ed9f00606e17d8da35402ffee
             75df78c3d31e0f889f012120c0862beb67753e7439ae242edb8373698356cf5a",
        ),
        (
            b"Jefe".to_vec(),
            b"what do ya want for nothing?".to_vec(),
            "51f4de5b33f249adf81aeb713a3c20f4fe631446fabdfa58244759ae58ef9009
             a99abf4eac2ca5fa87e692c440eb40023e7babb206d61de7b92f41529092b8fc",
        ),
        (
            vec![0xaa; 20],
            vec![0xdd; 50],
            "e1ac546ec4cb636f9976487be5c86be17a0252ca5d8d8df12cfb04735252 49ce
             9dd8d177ead710bc9b590547239107aef7b4abd43d87f0a68f1cbd9e2b6f7607",
        ),
    ];
    for (key, data, want) in cases {
        let want = h(want);
        let mut out = [0u8; 64];
        prf(&key, b"prefix", &data, &mut out);
        assert_eq!(out.to_vec(), want);
        // PRF-384, as the PTK uses, is the first 48 bytes of the same stream.
        let mut out384 = [0u8; 48];
        prf(&key, b"prefix", &data, &mut out384);
        assert_eq!(out384.to_vec(), want[..48].to_vec());
    }
}

// The handshake both scenarios run: the J.4.2 PMK, an AA above the SPA and an
// ANonce above the SNonce, so the Min/Max ordering of the PTK input is tested.
const AA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x01, 0x00];
const SPA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
const SNONCE: [u8; 32] = [0x11; 32];
const PTK: &str = "df3e300cab6142d40c893497162867f0d8a066384aaee26a5de3066ad63d36e8
                   30ab7cf57681b9d485e1fc58a9864986";

fn anonce() -> [u8; 32] {
    core::array::from_fn(|i| 0x20 + i as u8)
}

fn psk_ieee() -> [u8; 32] {
    pmk(b"password", b"IEEE")
}

// Message 1: version 2, Pairwise | Ack, key length 16, replay counter 1.
const M1: &str = "0203005f02008a00100000000000000001202122232425262728292a2b2c2d2e2f
                  303132333435363738393a3b3c3d3e3f000000000000000000000000000000000000
                  00000000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn the_ptk_matches_the_reference_in_either_address_order() {
    let pmk = psk_ieee();
    assert_eq!(ptk(&pmk, &AA, &SPA, &anonce(), &SNONCE).to_vec(), h(PTK));
    // The same PTK from the authenticator's side of the argument list.
    assert_eq!(Akm::Psk.derive_ptk(&pmk, &AA, &SPA, &anonce(), &SNONCE).to_vec(), h(PTK));
}

// A WPA2/WPA3 transition network: PSK and SAE, MFPC set, MFPR clear, the
// group management cipher named, and an RSNXE advertising hash-to-element.
const AP_RSNE_TRANSITION: &str = "301e0100000fac040100000fac040200000fac02000fac0880000000000fac06";
const AP_RSNXE: &str = "f40120";

// The transition network's handshake as the reference frames it. Message 2:
// header length 0x75, Key Information 0x010a, key length 0, MIC, the RSNE.
const M2_TRANSITION: &str = "0203007502010a0000000000000000000111111111111111111111111111111111111111
                             111111111111111111111111110000000000000000000000000000000000000000000000
                             0000000000000000004af97ec63ec8b241115b93068c3bf2d4001630140100000fac0401
                             00000fac040100000fac020000";
// Message 3: Install | Ack | MIC | Secure | Encrypted, replay 2, RSC 0x0705,
// key data wrapping the beacon's RSNE, its RSNXE, a GTK KDE (index 2) and the
// 0xDD 00 padding.
const M3_TRANSITION: &str = "020300a70213ca00100000000000000002202122232425262728292a2b2c2d2e2f303132
                             333435363738393a3b3c3d3e3f0000000000000000000000000000000005070000000000
                             000000000000000000949f297155d8e84760161b3a11bb571a00489d01f0fa9ec3810983
                             afbb3624fea4bd42005fa80bcbaf2fd9c470a9572f92c3072e08416553f5fa9326bc3612
                             dd88b12feeae341c6532e41ea19de6011f1536aeae929467a8a6fa";
// Message 4: Key Information 0x030a, replay 2, zero nonce, MIC, no key data.
const M4_TRANSITION: &str = "0203005f02030a0000000000000000000200000000000000000000000000000000000000
                             000000000000000000000000000000000000000000000000000000000000000000000000
                             00000000000000000051941757713f30e2c0c22b17c44dd6050000";

#[test]
fn a_transition_network_joined_with_psk_completes_against_the_reference() {
    let ap_rsne = h(AP_RSNE_TRANSITION);
    let ap_rsnxe = h(AP_RSNXE);
    let sel = select(&parse_rsne(&ap_rsne[2..]).unwrap(), true, JoinPolicy::PSK_ONLY).unwrap();
    assert_eq!((sel.akm, sel.pmf), (Akm::Psk, Pmf::Off));
    let own = station_rsne(&sel);
    assert_eq!(own, RSN_IE, "PSK without protection sends the plain WPA2 RSNE");

    let mut s = Supplicant::configure(&Config {
        pmk: psk_ieee(),
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: sel.akm,
        own_rsne: &own,
        own_rsnxe: None,
        ap_rsne: &ap_rsne,
        ap_rsnxe: Some(&ap_rsnxe),
        pmf: false,
    });
    let m2 = s.step(&h(M1)).reply.expect("message 2");
    assert_eq!(
        m2,
        h(M2_TRANSITION),
        "message 2: header length, Key Information 0x010a, key length 0, MIC, RSNE"
    );

    // Message 3, with four bytes of trailing junk (an FCS the driver kept).
    let mut m3 = h(M3_TRANSITION);
    m3.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
    let m4 = s.step(&m3).reply.expect("message 4");
    assert_eq!(s.state(), State::Connected);
    assert_eq!(
        m4,
        h(M4_TRANSITION),
        "message 4: Key Information 0x030a, replay 2, zero nonce, MIC, no key data"
    );
    assert_eq!(s.tk().to_vec(), h(PTK)[32..].to_vec());
    assert_eq!(s.gtk().to_vec(), (0xa0..0xb0).collect::<Vec<u8>>());
    assert_eq!((s.gtk_id(), s.gtk_rsc()), (2, 0x0705));
    assert!(s.igtk().is_none());
}

#[test]
fn a_psk_network_requiring_protection_delivers_the_igtk() {
    let ap_rsne = h("30140100000fac040100000fac040100000fac02cc00");
    let sel = select(&parse_rsne(&ap_rsne[2..]).unwrap(), false, JoinPolicy::PSK_ONLY).unwrap();
    assert_eq!((sel.akm, sel.pmf), (Akm::Psk, Pmf::Required));
    let own = station_rsne(&sel);
    assert_eq!(own.to_vec(), h("30140100000fac040100000fac040100000fac02c000"));

    let mut s = Supplicant::configure(&Config {
        pmk: psk_ieee(),
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: sel.akm,
        own_rsne: &own,
        own_rsnxe: None,
        ap_rsne: &ap_rsne,
        ap_rsnxe: None,
        pmf: true,
    });
    let m2 = s.step(&h(M1)).reply.expect("message 2");
    assert_eq!(
        m2,
        h("0203007502010a0000000000000000000111111111111111111111111111111111111111
           111111111111111111111111110000000000000000000000000000000000000000000000
           0000000000000000006967e07e8bb52e0dbac95d3732c8cfc9001630140100000fac0401
           00000fac040100000fac02c000")
    );
    let m3 = h("020300b70213ca00100000000000000002202122232425262728292a2b2c2d2e2f303132
        333435363738393a3b3c3d3e3f0000000000000000000000000000000005070000000000
        0000000000000000000d2e0b7319f60ddf39fe1d6160e8857b0058a6400fb441baf425dd
        e4876d18810e1ebf1a7f8f72d30de0e4c1956837c95bf7abbeeaa45b7a92fde4b3c31dea
        cf86f9efca197eb166d35cfe13dcb3f6b95eb7e8d48ffb2a09f38b26979d956559460af0
        223f49ebfb4975");
    let m4 = s.step(&m3).reply.expect("message 4");
    assert_eq!(s.state(), State::Connected);
    let k = parse(&m4).unwrap();
    assert_eq!(k.key_info, 0x030a);
    assert_eq!(s.gtk_id(), 1);
    let (igtk, id, ipn) = s.igtk().expect("the IGTK KDE");
    assert_eq!(igtk.to_vec(), (0xc0..0xd0).collect::<Vec<u8>>());
    assert_eq!((id, ipn), (4, [0x2a, 0, 0, 0, 0, 0]));
}

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
