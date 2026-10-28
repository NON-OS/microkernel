// NONOS Operating System (AGPL-3.0-or-later)

//! What the TY and MA images list beyond the AX211 image's versions: debug
//! commands, the CDB capability and capabilities the driver does not read;
//! and that each fits the grants with its own PNVM file.

use super::rig::{MA, MA4, MA4_PNVM, MA_PNVM, SO, TY, TY_PNVM};
use crate::gen3::cmds::{CAPA_CDB_SUPPORT, LONG_GROUP};
use crate::gen3::dram_map::classify;
use crate::gen3::layout::firmware_region_sizes;
use crate::gen3::plan::control_len_pnvm;
use crate::gen3::pnvm::reserve;
use crate::gen3::ucode::Ucode;

#[test]
fn what_differs_from_the_ax211_image_is_what_the_driver_does_not_read() {
    let (so, ty) = (Ucode::parse(SO).unwrap(), Ucode::parse(TY).unwrap());
    let (ma, ma4) = (Ucode::parse(MA).unwrap(), Ucode::parse(MA4).unwrap());
    for cmd in [0xF2, 0xF6] {
        assert_eq!(
            (so.cmd_version(LONG_GROUP, cmd), ma.cmd_version(LONG_GROUP, cmd)),
            (Some(7), Some(8))
        );
        assert_eq!(ma4.cmd_version(LONG_GROUP, cmd), Some(8));
    }
    assert_eq!((so.cmd_version(4, 0x0A), ma4.cmd_version(4, 0x0A)), (None, Some(1)));
    assert!(ma4.capa(CAPA_CDB_SUPPORT) && !ma.capa(CAPA_CDB_SUPPORT) && !so.capa(CAPA_CDB_SUPPORT));
    assert!(ty.capa(47) && !so.capa(47));
    for bit in [62, 106, 115] {
        assert!(so.capa(bit) && !ty.capa(bit), "{bit}");
    }
}

#[test]
fn every_image_fits_the_grants_and_its_pnvm_names_only_its_own_mac() {
    for (blob, pnvm, mac) in [(TY, TY_PNVM, 0x42u16), (MA, MA_PNVM, 0x44), (MA4, MA4_PNVM, 0x44)] {
        let u = Ucode::parse(blob).unwrap();
        assert!(!firmware_region_sizes(&classify(blob)).expect("64-page grants").is_empty());
        let room = reserve(pnvm, mac, 0x10D);
        assert!(room > 0, "{mac:#x}: the PNVM has sections for this MAC and GF");
        assert!(control_len_pnvm(u.iml.len(), room).is_some(), "{mac:#x}");
        assert_eq!(reserve(pnvm, 0x37, 0x10D), 0, "{mac:#x}: no section for SO");
    }
    assert_eq!(reserve(TY_PNVM, 0x44, 0x10D), 0, "the TY file names TY only");
}
