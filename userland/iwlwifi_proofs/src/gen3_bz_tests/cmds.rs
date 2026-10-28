// NONOS Operating System (AGPL-3.0-or-later)

//! The two join commands whose versions the BZ firmware moved, byte for
//! byte against Linux v6.12's structs. `iwl_phy_context_cmd` v5 (32 bytes):
//! `id_and_color` 0, `action` 4, `ci` 8 (`channel` 8, `band` 12, `width` 13,
//! `ctrl_pos` 14), `lmac_id` 16, `sbb_bandwidth` 20, `sbb_ctrl_channel_loc`
//! 21, `puncture_mask` 22 (v6 only), `dsp_cfg_flags` 24, reserved 28.
//! `iwl_link_config_cmd` v2 (208 bytes): v1's layout with `listen_lmac` (32)
//! reserved. A legacy link has no AP chandef, so the SBB bytes are zero.

use crate::gen3::station::ids::{check, ABSENT, APIS, PHY_CONTEXT_CMD};
use crate::gen3::station::link::{link_config, LinkParams, LINK_CONFIG_LEN};
use crate::gen3::station::phy::{phy_context, ACTION_ADD, PHY_CONTEXT_LEN};
use crate::gen3::ucode::Ucode;

use super::rig::GL;

#[test]
fn the_phy_context_is_the_v5_layout_with_no_sbb() {
    assert_eq!(PHY_CONTEXT_LEN, 32);
    let c = phy_context(ACTION_ADD, 36, false);
    assert_eq!(&c[0..4], &[0; 4], "PHY 0, color 0");
    assert_eq!(&c[4..8], &ACTION_ADD.to_le_bytes());
    assert_eq!(&c[8..12], &36u32.to_le_bytes());
    assert_eq!((c[12], c[13], c[14], c[15]), (0, 0, 0, 0), "5 GHz, 20 MHz, ctrl 0");
    assert_eq!(&c[16..20], &[0; 4], "LMAC 0");
    assert_eq!((c[20], c[21]), (0, 0), "sbb_bandwidth, sbb_ctrl_channel_loc");
    assert_eq!(&c[22..32], &[0; 10], "no puncture mask, no DSP flags");
    assert_eq!(phy_context(ACTION_ADD, 6, false)[12], 1, "2.4 GHz");
}

#[test]
fn the_link_context_leaves_the_v2_reserved_word_zero() {
    let p =
        LinkParams { addr: [2, 1, 2, 3, 4, 5], phy: Some(0), active: true, ..Default::default() };
    let c = link_config(2, &p);
    assert_eq!(c.len(), LINK_CONFIG_LEN);
    assert_eq!(LINK_CONFIG_LEN, 208);
    assert_eq!(&c[16..22], &p.addr);
    assert_eq!(&c[28..32], &1u32.to_le_bytes(), "active");
    assert_eq!(&c[32..36], &[0; 4], "listen_lmac in v1, reserved in v2");
    assert_eq!(c[166], 0, "spec_link_id");
}

#[test]
fn the_join_accepts_exactly_the_versions_it_encodes() {
    let u = Ucode::parse(GL).unwrap();
    assert_eq!(check(&u), Ok(()));
    let phy = APIS.iter().find(|a| a.cmd == PHY_CONTEXT_CMD && a.group == 1).unwrap();
    assert_eq!(phy.listed, &[4, 5], "v6 adds the puncture mask, not encoded");
    let link = APIS.iter().find(|a| (a.group, a.cmd) == (3, 0x09)).unwrap();
    assert_eq!(link.listed, &[ABSENT, 1, 2]);
}
