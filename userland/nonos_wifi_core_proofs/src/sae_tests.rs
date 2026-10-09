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

//! WPA3-SAE against published known answers, then the exchange end to end.
//!
//! Vector source: IEEE Std 802.11-2020, Annex J.10 (SAE test vectors), as
//! transcribed in hostapd's `src/common/common_module_tests.c`, function
//! `sae_tests()`, fetched from https://w1.fi/cgit/hostap/plain/src/common/
//! common_module_tests.c (sha256 a75c0bc7e9069a2c443cfd225e52c912493c0255448
//! 606948483d3e9e3bcd9e3). The hunting-and-pecking part derives the PWE for
//! addresses 4d:3f:2f:ff:e3:87 / a5:d8:aa:95:8e:3c and password
//! "mekmitasdigoat", builds the commit from the given rand and mask, and from
//! the peer's commit derives the KCK, PMK and PMKID. The hash-to-element part
//! derives PT for SSID "byteme", that password and identifier "psk4internet",
//! and from it the PWE for 00:09:5b:66:ec:1e / 00:0b:6b:d9:02:46.

use nonos_wifi_core::sae::commit::Commit;
use nonos_wifi_core::sae::frame::{
    commit_body, parse_commit, parse_token_request, STATUS_SAE_HASH_TO_ELEMENT, STATUS_SUCCESS,
};
use nonos_wifi_core::sae::group::point_xy;
use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeFailure, SaeState, SaeStation, SaeStep};

fn h(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

fn a32(v: &[u8]) -> [u8; 32] {
    v.try_into().unwrap()
}

const ADDR1: [u8; 6] = [0x4d, 0x3f, 0x2f, 0xff, 0xe3, 0x87];
const ADDR2: [u8; 6] = [0xa5, 0xd8, 0xaa, 0x95, 0x8e, 0x3c];
const PW: &[u8] = b"mekmitasdigoat";

const LOCAL_RAND: &str = "992465fd3daa3c60aa6565b7f62a2a7f2e12dd12f198faf4fbed89d7ff1ace94";
const LOCAL_MASK: &str = "9507a90f777a044d6a0830b91ea3d5dd70bece44e1acffb86983b5e1bf9fb322";
const LOCAL_COMMIT: &str = "
    1300
    2e2c0f0db52440ad146d967114ce005ce1eab0aa2c2e5c2871b774f6c2575c65
    d5ad9e00829707aa36ba8b859738fc961d08243505f47c035376d7ac4bc8d7b9
    5083bf43827d0fc31ed778dd3671fd21a46d1091d64b6f9a1e1272621325dbe1";
const PEER_COMMIT: &str = "
    1300
    591b96f3397fb945100848e7b550543b6720d88337ee93fc49fd6df7e08b5223
    e71b9bb048d3873f20556953a96c91536fd8ee6ca9b4a68a148b056a909be03e
    83ae208f60f8ef5537858074db06687032399862999b511e0a1552a5fea317c2";
const KCK: &str = "1e733f6d9bd53256287304338831b09a39406d121017073a5c30db36f36cb81a";
const PMK: &str = "4e4dfab1a2dd8ac1a91790f953faaa452ae5c6873ab75b63605ba663f8a7fe59";
const PMKID: &str = "8747a600eea3f9f22475df58ca1e5498";

fn hnp_commit() -> Commit {
    // The stand-in password only has to be as long as the real one.
    let stub = [0x5Au8; 14];
    let pwe = derive_pwe(PW, &stub, &ADDR1, &ADDR2).expect("a PWE is found");
    Commit::new(pwe, &a32(&h(LOCAL_RAND)), &a32(&h(LOCAL_MASK))).expect("rand and mask are valid")
}

#[test]
fn hunting_and_pecking_gives_the_annex_j10_commit() {
    let c = hnp_commit();
    let body = commit_body(&c.scalar(), &c.element().unwrap(), &[], false);
    assert_eq!(body, h(LOCAL_COMMIT), "group, commit-scalar and COMMIT-ELEMENT");
}

#[test]
fn the_annex_j10_peer_commit_gives_its_kck_pmk_and_pmkid() {
    let c = hnp_commit();
    let peer = parse_commit(&h(PEER_COMMIT)).expect("the peer commit parses");
    let keys = c.derive_keys(&peer.scalar, &peer.element).expect("K is not the identity");
    assert_eq!(keys.kck.to_vec(), h(KCK), "KCK");
    assert_eq!(keys.pmk.to_vec(), h(PMK), "PMK");
    assert_eq!(keys.pmkid.to_vec(), h(PMKID), "PMKID");
}

#[test]
fn hash_to_element_gives_the_annex_j10_pwe() {
    let pt = derive_pt(b"byteme", PW, Some(b"psk4internet")).expect("PT");
    let a = [0x00, 0x09, 0x5b, 0x66, 0xec, 0x1e];
    let b = [0x00, 0x0b, 0x6b, 0xd9, 0x02, 0x46];
    let pwe = pwe_from_pt(&pt, &a, &b).expect("PWE");
    let xy = point_xy(&pwe).unwrap();
    assert_eq!(
        xy[..32].to_vec(),
        h("c93049b9e64000f848201649e999f2b5c22dea69b5632c9df4d633b8aa1f6c1e"),
        "PWE.x"
    );
    assert_eq!(
        xy[32..].to_vec(),
        h("73634e94b53d82e7383a8d258199d9dc1a5ee8269d060382ccbf33e614ff59a0"),
        "PWE.y"
    );
    // The PWE does not depend on which station derives it.
    assert_eq!(point_xy(&pwe_from_pt(&pt, &b, &a).unwrap()).unwrap(), xy);
}

// Run a whole exchange: the station against an AP built from the same code
// with its own rand and mask. Both must end Accepted with the same PMK.
fn exchange(h2e: bool, sta_pw: &[u8], ap_pw: &[u8]) -> (SaeStation, SaeStation) {
    let (sta_mac, ap_mac) = ([0x02, 0, 0, 0, 0, 1], [0x02, 0, 0, 0, 0, 2]);
    let pwe = |pw: &[u8]| {
        if h2e {
            pwe_from_pt(&derive_pt(b"home", pw, None).unwrap(), &sta_mac, &ap_mac).unwrap()
        } else {
            derive_pwe(pw, &vec![7u8; pw.len()], &sta_mac, &ap_mac).unwrap()
        }
    };
    let mut sta = SaeStation::start(pwe(sta_pw), h2e, &[0x11; 32], &[0x22; 32]).unwrap();
    let mut ap = SaeStation::start(pwe(ap_pw), h2e, &[0x33; 32], &[0x44; 32]).unwrap();
    let (_, status, sta_commit) = sta.commit_frame();
    let want = if h2e { STATUS_SAE_HASH_TO_ELEMENT } else { STATUS_SUCCESS };
    assert_eq!(status, want, "the commit's status code names the PWE method");
    let (_, ap_status, ap_commit) = ap.commit_frame();
    let SaeStep::Send { body: ap_confirm, .. } = ap.on_frame(1, status, &sta_commit) else {
        panic!("the AP answers a commit with a confirm");
    };
    let SaeStep::Send { seq, body: sta_confirm, .. } = sta.on_frame(1, ap_status, &ap_commit) else {
        panic!("the station answers the AP's commit with a confirm");
    };
    assert_eq!(seq, 2);
    ap.on_frame(2, 0, &sta_confirm);
    sta.on_frame(2, 0, &ap_confirm);
    (sta, ap)
}

#[test]
fn an_exchange_with_the_same_password_agrees_on_the_pmk() {
    for h2e in [false, true] {
        let (sta, ap) = exchange(h2e, b"correct horse", b"correct horse");
        assert_eq!(sta.state(), SaeState::Accepted, "h2e={h2e}");
        assert_eq!(ap.state(), SaeState::Accepted, "h2e={h2e}");
        assert_eq!(sta.pmk().unwrap(), ap.pmk().unwrap(), "same PMK and PMKID, h2e={h2e}");
    }
}

#[test]
fn a_different_password_fails_at_the_confirm() {
    for h2e in [false, true] {
        let (sta, _) = exchange(h2e, b"correct horse", b"wrong horse!!");
        assert_eq!(sta.state(), SaeState::Failed, "h2e={h2e}");
        assert_eq!(sta.failure(), Some(SaeFailure::BadConfirm));
        assert!(sta.pmk().is_none(), "no PMK from a failed exchange");
    }
}

#[test]
fn a_reflected_commit_is_dropped_and_a_mismatched_method_refused() {
    let pwe = derive_pwe(PW, &[1u8; 14], &ADDR1, &ADDR2).unwrap();
    let mut sta = SaeStation::start(pwe, false, &[0x11; 32], &[0x22; 32]).unwrap();
    let (_, status, own) = sta.commit_frame();
    assert!(matches!(sta.on_frame(1, status, &own), SaeStep::Ignore), "reflection dropped");
    assert_eq!(sta.state(), SaeState::Committed);
    // The AP answering a hunting-and-pecking commit with the H2E status code.
    let c = hnp_commit();
    let other = commit_body(&c.scalar(), &c.element().unwrap(), &[], false);
    assert!(matches!(sta.on_frame(1, STATUS_SAE_HASH_TO_ELEMENT, &other), SaeStep::Failed));
    assert_eq!(sta.failure(), Some(SaeFailure::BadCommit));
}

#[test]
fn an_anti_clogging_request_is_answered_with_the_token() {
    for h2e in [false, true] {
        let pwe = derive_pwe(PW, &[1u8; 14], &ADDR1, &ADDR2).unwrap();
        let mut sta = SaeStation::start(pwe, h2e, &[0x11; 32], &[0x22; 32]).unwrap();
        let token = [0xABu8; 34];
        let mut req = vec![0x13, 0x00];
        if h2e {
            req.extend_from_slice(&[255, 35, 93]);
        }
        req.extend_from_slice(&token);
        let SaeStep::Send { seq, body, .. } = sta.on_frame(1, 76, &req) else {
            panic!("the commit is sent again");
        };
        assert_eq!(seq, 1);
        assert!(body.windows(token.len()).any(|w| w == token), "the token rides along, h2e={h2e}");
        assert_eq!(body.len(), 2 + 96 + 34 + if h2e { 3 } else { 0 });
    }
}

#[test]
fn malformed_commits_are_refused() {
    let good = h(PEER_COMMIT);
    assert!(parse_commit(&good).is_some());
    assert!(parse_commit(&good[..good.len() - 1]).is_none(), "short");
    let mut g = good.clone();
    g[0] = 20;
    assert!(parse_commit(&g).is_none(), "group 20 is not run");
    for scalar in [[0u8; 32], {
        let mut one = [0u8; 32];
        one[31] = 1;
        one
    }, [0xFF; 32]] {
        let mut s = good.clone();
        s[2..34].copy_from_slice(&scalar);
        assert!(parse_commit(&s).is_none(), "scalar must be in (1, r)");
    }
    let mut off = good.clone();
    off[97] ^= 1; // flip a bit of y: no longer on the curve
    assert!(parse_commit(&off).is_none(), "element off the curve");
    let mut big = good.clone();
    big[34..66].copy_from_slice(&[0xFF; 32]);
    assert!(parse_commit(&big).is_none(), "x not below p");
    // Token requests: wrong group, empty, a container that runs past the body.
    assert!(parse_token_request(&[0x14, 0x00, 1, 2], false).is_none());
    assert!(parse_token_request(&[0x13, 0x00], false).is_none());
    assert!(parse_token_request(&[0x13, 0x00, 255, 40, 93, 1, 2], true).is_none());
}

/// The group's constants, decoded from hex at compile time, against what
/// holds between them and the published P-256 values: 2^256 mod p plus p
/// wraps to zero, and b and the order read back as published.
#[test]
fn the_group_constants_are_p256s() {
    use nonos_wifi_core::sae::group::{CURVE_B, ORDER, PRIME, TWO_256_MOD_P};
    let mut carry = 0u16;
    for i in (0..32).rev() {
        let s = PRIME[i] as u16 + TWO_256_MOD_P[i] as u16 + carry;
        assert_eq!(s & 0xff, 0, "byte {i} of p + (2^256 mod p)");
        carry = s >> 8;
    }
    assert_eq!(carry, 1, "p + (2^256 mod p) is 2^256");
    assert_eq!(a32(&h("ffffffff00000001000000000000000000000000ffffffffffffffffffffffff")), PRIME);
    assert_eq!(a32(&h("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551")), ORDER);
    assert_eq!(a32(&h("5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b")), CURVE_B);
}
