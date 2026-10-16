// NONOS Operating System (AGPL-3.0-or-later)
//! The link context, its rates, and the rate word.

use crate::gen3::cmds::link_config_add;
use crate::gen3::station::link::{link_config, LinkParams, MODIFY_ACTIVE, MODIFY_RATES_INFO};
use crate::gen3::station::phy::{ACTION_ADD, ACTION_MODIFY};
use crate::gen3::station::rates::{bss_rates, rate_n_flags, BssRates};
use super::fixtures::{ADDR, le32};

#[test]
fn the_link_context_add_matches_the_bring_up_and_a_change_says_what_changed() {
    let add = link_config(ACTION_ADD, &LinkParams { addr: ADDR, ..LinkParams::default() });
    assert_eq!(add, link_config_add(ADDR));
    let p = LinkParams {
        addr: ADDR,
        phy: Some(0),
        active: true,
        modify_mask: MODIFY_ACTIVE | MODIFY_RATES_INFO,
        cck_ack: 0x0F,
        ofdm_ack: 0x15,
        short_preamble: true,
        short_slot: true,
        beacon_int: 100,
        dtim_period: 3,
    };
    let mut want = vec![0u8; 208];
    want[0] = 2;
    want[16..22].copy_from_slice(&ADDR);
    want[24] = 3;
    want[28] = 1;
    want[36] = 0x0F;
    want[40] = 0x15;
    want[44] = 1;
    want[48] = 1;
    want[136] = 100;
    want[140..142].copy_from_slice(&300u16.to_le_bytes());
    assert_eq!(link_config(ACTION_MODIFY, &p), want);
    let off = link_config(ACTION_MODIFY, &LinkParams { active: false, modify_mask: MODIFY_ACTIVE, ..p });
    assert_eq!((le32(&off, 12), le32(&off, 24), le32(&off, 28)), (0, 1, 0), "deactivated, PHY kept");
}

#[test]
fn ack_rates_add_the_mandatory_rates_below_the_lowest_basic_one() {
    // An 802.11b/g AP: 1, 2, 5.5 and 11 basic, the OFDM rates not.
    let bg = [0x82, 0x84, 0x8B, 0x96, 0x0C, 0x12, 0x18, 0x24, 0x30, 0x48, 0x60, 0x6C];
    assert_eq!(bss_rates(&bg, 6), BssRates { cck_ack: 0x0F, ofdm_ack: 0x15, mgmt: 0, data: 3 });
    // A g-only AP: 6, 12 and 24 basic; CCK all added as mandatory.
    let g = [0x8C, 0x12, 0x98, 0x24, 0xB0, 0x48, 0x60, 0x6C];
    assert_eq!(bss_rates(&g, 1), BssRates { cck_ack: 0x0F, ofdm_ack: 0x15, mgmt: 4, data: 8 });
    // 11 basic only: 1, 2 and 5.5 below it are added.
    assert_eq!(bss_rates(&[0x96, 0x0C], 11).cck_ack, 0x0F);
    // 5 GHz with 12 and 24 basic: 6 is mandatory below; a CCK octet is not a
    // rate there.
    let a = [0x82, 0x98, 0xB0, 0x48];
    assert_eq!(bss_rates(&a, 36), BssRates { cck_ack: 0x0F, ofdm_ack: 0x15, mgmt: 6, data: 8 });
    // 54 basic: 6, 12 and 24 below it.
    assert_eq!(bss_rates(&[0xEC], 36).ofdm_ack, 0x80 | 0x15);
    // No basic rate at all: the lowest of the band.
    assert_eq!((bss_rates(&[0x0C], 1).mgmt, bss_rates(&[0x0C], 40).mgmt), (0, 4));
}

#[test]
fn the_rate_word_is_the_version_2_legacy_format() {
    assert_eq!(rate_n_flags(0, 0x3), 0x0000_4000, "1 Mb/s CCK on antenna A");
    assert_eq!(rate_n_flags(3, 0x3), 0x0000_4003, "11 Mb/s CCK");
    assert_eq!(rate_n_flags(4, 0x3), 0x0000_4100, "6 Mb/s OFDM");
    assert_eq!(rate_n_flags(8, 0x2), 0x0000_8104, "24 Mb/s OFDM on antenna B");
    assert_eq!(rate_n_flags(11, 0x1), 0x0000_4107, "54 Mb/s OFDM");
    assert_eq!(rate_n_flags(99, 0x0), 0x0000_4107, "an index past 54 Mb/s is 54, no antenna is A");
}
