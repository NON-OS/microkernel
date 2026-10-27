// NONOS Operating System (AGPL-3.0-or-later)

//! The bundled BE200 and BE201 images and their PNVM files, read by the
//! driver's own parsers: every image parses, has the MLD API and scan
//! request 17, and passes the join layout check; the versions that moved
//! from the SO image are the ones the driver now encodes; the images fit
//! the broker's grants and their PNVM sections fit the control region.

use super::rig::{BZ, BZ_PNVM, GL, GL_PNVM};
use crate::firmware::blob::{gen3_blob, gen3_pnvm};
use crate::firmware::carried::runs;
use crate::gen3::cmds::{CAPA_MLD_API_SUPPORT, LONG_GROUP, MCC_UPDATE_CMD, SCAN_REQ_UMAC};
use crate::gen3::dram_map::classify;
use crate::gen3::layout::firmware_region_sizes;
use crate::gen3::plan::control_len_pnvm;
use crate::gen3::pnvm;
use crate::gen3::select::Image;
use crate::gen3::station::ids::check;
use crate::gen3::ucode::Ucode;

#[test]
fn both_images_parse_and_speak_every_join_layout() {
    for (blob, mac) in [(GL, 0x47u16), (BZ, 0x46)] {
        let u = Ucode::parse(blob).expect("parses");
        assert!(u.capa(CAPA_MLD_API_SUPPORT), "{mac:#x}");
        assert_eq!(u.cmd_version(LONG_GROUP, SCAN_REQ_UMAC), Some(17));
        assert_eq!(check(&u), Ok(()), "{mac:#x}");
        // PHY_CONTEXT v5, LINK_CONFIG v2, the MCC reply v8, ALIVE v6.
        assert_eq!(u.cmd_version(LONG_GROUP, 0x08), Some(5));
        assert_eq!(u.cmd_version(3, 0x09), Some(2));
        assert_eq!(u.notif_version(LONG_GROUP, MCC_UPDATE_CMD), Some(8));
        assert_eq!(u.notif_version(0, 0x01), Some(6));
        let sizes = firmware_region_sizes(&classify(blob)).expect("fits 64-page grants");
        assert!(!sizes.is_empty());
        let reserve = pnvm::reserve(if mac == 0x47 { GL_PNVM } else { BZ_PNVM }, mac, 0x112);
        assert!(reserve > 0, "{mac:#x}: the PNVM has sections for this MAC and FM");
        assert!(control_len_pnvm(u.iml.len(), reserve).is_some(), "{mac:#x}");
    }
}

#[test]
fn the_tree_runs_every_be200_and_be201_id() {
    for id in [0x272Bu16, 0xA840, 0x7740, 0x4D40] {
        assert!(runs(id), "{id:#06x}");
    }
    let gl = gen3_blob(Image::GlFm).unwrap();
    let bz = gen3_blob(Image::BzFm).unwrap();
    assert_eq!((gl.name, gl.bytes.len()), ("iwlwifi-gl-c0-fm-c0-92.ucode", GL.len()));
    assert_eq!((bz.name, bz.bytes.len()), ("iwlwifi-bz-b0-fm-c0-92.ucode", BZ.len()));
    assert_eq!(gen3_pnvm(Image::GlFm).map(<[u8]>::len), Some(GL_PNVM.len()));
    assert_eq!(gen3_pnvm(Image::BzFm).map(<[u8]>::len), Some(BZ_PNVM.len()));
}

#[test]
fn a_pnvm_has_no_section_for_another_mac() {
    assert_eq!(pnvm::reserve(GL_PNVM, 0x46, 0x112), 0, "the Gl file names Gl only");
    assert_eq!(pnvm::reserve(BZ_PNVM, 0x47, 0x112), 0, "the Bz file names Bz only");
}
