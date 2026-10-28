// NONOS Operating System (AGPL-3.0-or-later)

//! The transmit power limits as `mt76_connac_mcu_rate_txpower_band` lays
//! them out for the MT7921 family with no device tree, SAR or ACPI table:
//! eight channels a message, Linux's channel lists, the band byte, the last
//! message marked, 161 limits per channel built as
//! `mt76_connac_mcu_build_sku` builds them; 20 dBm on every channel the world
//! domain allows, 127 on the rest.

use crate::init::caps::{Caps, DEFAULT};
use crate::init::power::bodies;

fn chan(b: &[u8], i: usize) -> (u8, &[u8]) {
    let at = 44 + 162 * i;
    (b[at], &b[at + 1..at + 162])
}

#[test]
fn both_bands_go_in_eight_channel_messages_the_last_marked() {
    let all = bodies(DEFAULT);
    assert_eq!(all.len(), 2 + 6);
    let heads: Vec<[u8; 4]> = all.iter().map(|b| [b[4], b[5], b[6], b[8]]).collect();
    assert_eq!(heads[..2], [[8, 1, 0, b'0'], [6, 1, 0, b'0']], "2.4 GHz: 1..8, 9..14");
    assert_eq!(heads[7], [8, 2, 1, b'0'], "5 GHz, the message ending at 177 is the last");
    assert!(heads[2..7].iter().all(|h| h[1] == 2 && h[2] == 0));
    assert!(all.iter().all(|b| b.len() == 44 + 162 * b[4] as usize && b[9] == b'0'));
    assert_eq!((chan(&all[1], 5).0, chan(&all[7], 7).0), (14, 177));
    let only2 = bodies(Caps { ghz5: false, ..DEFAULT });
    assert_eq!((only2.len(), only2[1][6]), (2, 1), "without 5 GHz, channel 14 ends it");
}

#[test]
fn every_rate_is_held_to_the_world_limit_where_the_world_allows() {
    let all = bodies(DEFAULT);
    let (ch, six) = chan(&all[0], 5);
    assert_eq!(ch, 6);
    let mut want = [40u8; 161];
    (0..4).for_each(|g| want[29 + 12 * g + 10..29 + 12 * g + 12].fill(127));
    assert_eq!(six, want, "CCK, OFDM, HT, VHT ten of twelve, HE");
    let (ch, c36) = chan(&all[2], 0);
    want[..4].fill(127);
    assert_eq!((ch, c36), (36, &want[..]), "no CCK limit on 5 GHz");
    let (ch, c38) = chan(&all[2], 1);
    assert_eq!((ch, c38.iter().all(|&v| v == 127)), (38, true), "a 40 MHz centre: no limit");
}
