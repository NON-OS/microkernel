// NONOS Operating System (AGPL-3.0-or-later)

//! The access point's station record, TLV by TLV against
//! `mt76_connac_mcu_sta_cmd`: the request header and its TLV count; basic
//! (`CONNECTION_INFRA_AP`, `PORT_SECURE`, `EXTRA_INFO_NEW` only on a record
//! made); PHY, RA and state when enabled; the WTBL request with its nested
//! count and its growing length; and the record removed.

use crate::vif::sta::{body, Peer, State};

const AP: [u8; 6] = [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

fn peer(ghz5: bool) -> Peer {
    let rates = if ghz5 { (0xff, 0x15) } else { (0xfff, 0xf) };
    Peer { addr: AP, wcid: 1, aid: 3, ghz5, rates }
}

fn le16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

#[test]
fn a_new_record_carries_basic_phy_ra_state_and_the_wtbl() {
    let b = body(&peer(false), Some(State::None));
    assert_eq!(&b[..8], &[0, 1, 5, 0, 1, 0, 0, 0], "bss 0, wcid 1, five TLVs, appended");
    let t = 8;
    assert_eq!((le16(&b, t), le16(&b, t + 2)), (0, 20));
    assert_eq!(u32::from_le_bytes(b[t + 4..t + 8].try_into().unwrap()), 0x1_0002);
    assert_eq!((b[t + 8], le16(&b, t + 10), le16(&b, t + 18)), (2, 3, 3), "secure, aid, new");
    assert_eq!(&b[t + 12..t + 18], &AP);
    let p = t + 20;
    assert_eq!((le16(&b, p), le16(&b, p + 4), b[p + 6], b[p + 9]), (0x15, 0xf, 0x3, 220));
    let r = p + 12;
    assert_eq!((le16(&b, r), le16(&b, r + 4)), (1, 0xff << 6 | 0xf), "OFDM 13:6, CCK 3:0");
    let s = r + 16;
    assert_eq!((le16(&b, s), b[s + 8]), (7, 0), "STA_REC_STATE, none");
    let w = s + 12;
    assert_eq!((le16(&b, w), le16(&b, w + 2), w + 52), (13, 52, b.len()), "WTBL, grown to 52");
    assert_eq!(&b[w + 4..w + 12], &[1, 1, 3, 0, 0, 0, 0, 0], "wcid, RESET_AND_SET, 3 nested");
    let g = w + 12;
    assert_eq!((&b[g + 4..g + 10], b[g + 10], le16(&b, g + 16)), (&AP[..], 0, 3));
    assert_eq!(&b[g + 20..g + 28], &[1, 0, 12, 0, 0, 1, 1, 1], "WTBL_RX: rca1, rca2, rv");
    assert_eq!(&b[g + 32..g + 39], &[6, 0, 8, 0, 1, 0, 1], "WTBL_HDR_TRANS: to-DS, no rx trans");
}

#[test]
fn the_association_update_is_not_new_and_a_removal_is_disconnected() {
    let a = body(&peer(true), Some(State::Assoc));
    assert_eq!((le16(&a, 8 + 18), a[8 + 20 + 6], le16(&a, 8 + 32 + 4)), (1, 8, 0xff << 6));
    assert_eq!(a[8 + 48 + 8], 2, "STA_REC_STATE: associated");
    let gone = body(&peer(false), None);
    assert_eq!((gone[2], gone.len()), (2, 8 + 20 + 12), "basic and an empty WTBL");
    assert_eq!((gone[8 + 8], le16(&gone, 8 + 18)), (0, 1), "disconnected, not new");
    assert_eq!(&gone[28..40], &[13, 0, 12, 0, 1, 1, 0, 0, 0, 0, 0, 0]);
}
