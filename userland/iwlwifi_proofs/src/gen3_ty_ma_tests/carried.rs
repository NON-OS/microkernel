// NONOS Operating System (AGPL-3.0-or-later)

//! The tree runs the discrete AX210 and Meteor Lake, names their images,
//! and a dual-radio Meteor Lake module puts a 5 GHz PHY on LMAC 1.

use super::rig::{MA, MA4, MA4_PNVM, MA_PNVM, TY, TY_PNVM};
use crate::firmware::blob::{gen3_blob, gen3_pnvm};
use crate::firmware::carried::runs;
use crate::firmware::generation::name;
use crate::gen3::select::Image;
use crate::gen3::station::phy::{phy_context, ACTION_ADD};

#[test]
fn the_tree_runs_the_discrete_ax210_and_meteor_lake() {
    for id in [0x2725u16, 0x7E40, 0x2729] {
        assert!(runs(id), "{id:#06x}");
        assert!(name(id).unwrap().boots, "{id:#06x}");
    }
    assert!(name(0x2725).unwrap().what.contains("discrete AX210"));
    assert!(name(0x7E40).unwrap().what.contains("Meteor Lake"));
    for (image, file, blob, pnvm) in [
        (Image::TyGf, "iwlwifi-ty-a0-gf-a0-86.ucode", TY, TY_PNVM),
        (Image::MaGf, "iwlwifi-ma-b0-gf-a0-86.ucode", MA, MA_PNVM),
        (Image::MaGf4, "iwlwifi-ma-b0-gf4-a0-86.ucode", MA4, MA4_PNVM),
    ] {
        let b = gen3_blob(image).unwrap();
        assert_eq!((b.name, b.bytes), (file, blob));
        assert_eq!(gen3_pnvm(image), Some(pnvm), "{file}");
    }
}

#[test]
fn a_dual_radio_firmware_runs_a_5_ghz_phy_on_lmac_1() {
    assert_eq!(phy_context(ACTION_ADD, 36, true)[16..20], [1, 0, 0, 0], "5 GHz, CDB");
    assert_eq!(phy_context(ACTION_ADD, 6, true)[16..20], [0; 4], "2.4 GHz, CDB");
    assert_eq!(phy_context(ACTION_ADD, 36, false)[16..20], [0; 4], "5 GHz, one LMAC");
    let mut cdb = phy_context(ACTION_ADD, 149, true);
    cdb[16] = 0;
    assert_eq!(cdb, phy_context(ACTION_ADD, 149, false), "nothing else changes");
}
