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
use super::SPOKEN;
use crate::gen3::cmds::{CAPA_MLD_API_SUPPORT, CAPA_UMAC_SCAN};
use crate::gen3::station::ids::{check, APIS};
use crate::gen3::ucode::Ucode;

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
