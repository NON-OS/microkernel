// NONOS Operating System (AGPL-3.0-or-later)

//! An RSNE parsed with its defaults, and a malformed one refused.

use nonos_wifi_core::rsn::parse_rsne;
use nonos_wifi_core::rsn::suite::{CIPHER_BIP_CMAC_128, CIPHER_CCMP};

// An RSNE body: version 1, group, pairwise list, AKM list, caps.
pub(super) fn body(group: u8, pairwise: &[u8], akms: &[u8], caps: u16) -> Vec<u8> {
    let mut b = vec![1, 0, 0x00, 0x0f, 0xac, group];
    b.extend_from_slice(&(pairwise.len() as u16).to_le_bytes());
    for p in pairwise {
        b.extend_from_slice(&[0x00, 0x0f, 0xac, *p]);
    }
    b.extend_from_slice(&(akms.len() as u16).to_le_bytes());
    for a in akms {
        b.extend_from_slice(&[0x00, 0x0f, 0xac, *a]);
    }
    b.extend_from_slice(&caps.to_le_bytes());
    b
}

#[test]
fn an_rsne_is_parsed_with_its_defaults() {
    let r = parse_rsne(&body(4, &[2, 4], &[2, 8], 0x00C0)).unwrap();
    assert_eq!(r.group, CIPHER_CCMP);
    assert!(r.pairwise_ccmp && r.akm_psk && r.akm_sae && !r.akm_psk_sha256);
    assert!(r.mfpc() && r.mfpr());
    // Only the version: CCMP everywhere and 802.1X, which this station lacks.
    let v = parse_rsne(&[1, 0]).unwrap();
    assert!(v.pairwise_ccmp && !v.akm_psk && !v.akm_sae);
    // The group management cipher after an empty PMKID list.
    let mut b = body(4, &[4], &[8], 0x00C0);
    b.extend_from_slice(&[0, 0, 0x00, 0x0f, 0xac, 6]);
    assert_eq!(parse_rsne(&b).unwrap().group_mgmt, Some(CIPHER_BIP_CMAC_128));
}

#[test]
fn a_malformed_rsne_is_refused() {
    assert!(parse_rsne(&[]).is_none(), "empty");
    assert!(parse_rsne(&[2, 0]).is_none(), "version 2");
    let mut b = body(4, &[4], &[2], 0);
    b[6] = 9; // pairwise count 9 with one suite present
    assert!(parse_rsne(&b).is_none(), "a list past the end");
    assert!(parse_rsne(&[1, 0, 0x00, 0x0f, 0xac]).is_none(), "a cut suite");
    let mut p = body(4, &[4], &[2], 0);
    p.extend_from_slice(&[3, 0, 1, 2]); // three PMKIDs announced, none present
    assert!(parse_rsne(&p).is_none(), "a PMKID list past the end");
}
