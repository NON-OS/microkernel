// NONOS Operating System (AGPL-3.0-or-later)

//! The carried TY and MA images read by the driver's own parser. They are
//! the AX211 image's build (core83-73) for other MACs: every command and
//! notification the driver speaks has the AX211 image's version in each, so
//! the bring-up and the join run unchanged. What differs is pinned: the MA
//! images list two debug commands at version 8 (LONG 0xF2, LDBG_CONFIG_CMD
//! 0xF6), the GF4 image the CDB capability and a debug command (group 4,
//! 0x0A), and the TY image capabilities nothing here reads (FTM_CALIBRATED
//! set; RFIM, SYNCED_TIME and 115 clear).

use super::rig::{MA, MA4, SO, TY};
use crate::gen3::cmds::*;
use crate::gen3::station::ids::{check, APIS};
use crate::gen3::ucode::Ucode;

/// What the bring-up and the scan send or read, beside the join's `APIS`.
const SPOKEN: &[(u8, u8)] = &[
    (SYSTEM_GROUP, INIT_EXTENDED_CFG_CMD),
    (REGULATORY_AND_NVM_GROUP, NVM_ACCESS_COMPLETE),
    (REGULATORY_AND_NVM_GROUP, NVM_GET_INFO),
    (REGULATORY_AND_NVM_GROUP, PNVM_INIT_COMPLETE_NTFY),
    (SYSTEM_GROUP, SOC_CONFIGURATION_CMD),
    (LONG_GROUP, TX_ANT_CONFIGURATION_CMD),
    (LONG_GROUP, BT_CONFIG),
    (LONG_GROUP, POWER_TABLE_CMD),
    (LONG_GROUP, MCC_UPDATE_CMD),
    (LONG_GROUP, SCAN_CFG_CMD),
    (LONG_GROUP, SCAN_REQ_UMAC),
    (LONG_GROUP, SCAN_COMPLETE_UMAC),
    (MAC_CONF_GROUP, MAC_CONFIG_CMD),
    (MAC_CONF_GROUP, LINK_CONFIG_CMD),
    (MAC_CONF_GROUP, 0xF6),
    (MAC_CONF_GROUP, 0xFB),
    (LEGACY_GROUP, 0x01),
    (LEGACY_GROUP, REPLY_RX_MPDU_CMD),
    (LEGACY_GROUP, 0x1C),
];

fn versions(u: &Ucode<'_>, g: u8, c: u8) -> (Option<u8>, Option<u8>) {
    (u.cmd_version(g, c), u.notif_version(g, c))
}

#[test]
fn every_version_the_driver_speaks_is_the_ax211_images() {
    let so = Ucode::parse(SO).unwrap();
    let all = SPOKEN.iter().copied().chain(APIS.iter().map(|a| (a.group, a.cmd)));
    for (g, c) in all {
        for (name, blob) in [("ty", TY), ("ma", MA), ("ma gf4", MA4)] {
            let u = Ucode::parse(blob).expect("parses");
            assert_eq!(versions(&u, g, c), versions(&so, g, c), "{name} {g:#x}/{c:#04x}");
        }
    }
    for blob in [TY, MA, MA4] {
        let u = Ucode::parse(blob).unwrap();
        assert!(u.capa(CAPA_MLD_API_SUPPORT) && u.capa(CAPA_UMAC_SCAN));
        assert_eq!(check(&u), Ok(()), "the join's layouts");
    }
}
