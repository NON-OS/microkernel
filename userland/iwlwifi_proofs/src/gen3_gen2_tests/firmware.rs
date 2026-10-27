// NONOS Operating System (AGPL-3.0-or-later)

//! The carried 22000-family images read by the driver's own parser. They
//! carry no image loader (the gen3 parse refuses them; the family's takes
//! them), speak the MLD API and pass the join's layout check, and fit the
//! grants. Against the AX211 image, of every version the driver speaks
//! exactly these differ, each handled: TX_CMD 9 (the gen2 command, by
//! family), the RX MPDU notification 4 (the version 1 descriptor, by
//! family), SCAN_REQ_UMAC 15 (the band in `v2.band`), POWER_TABLE_CMD 6,
//! SESSION_PROTECTION_CMD 1 and MAC_CONFIG_CMD 1 (the same bytes; the AX200
//! build lists no MAC_CONFIG_CMD), the MCC reply 5 and the session
//! notification 2 (read as before).

use super::rig::{CC, QUZ, QU_B, QU_C, SO};
use crate::gen3::dev::Speaks;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::firmware_region_sizes;
use crate::gen3::station::ids::{check, APIS};
use crate::gen3::ucode::Ucode;
use crate::tests_gen3::gen3_ty_ma_tests::SPOKEN;

type Vers = (Option<u8>, Option<u8>);
const DIFFER: [(u8, u8, Vers, Vers); 9] = [
    (0, 0x1C, (Some(10), None), (Some(9), None)),
    (0, 0xC1, (None, Some(5)), (None, Some(4))),
    (1, 0x0D, (Some(17), Some(0)), (Some(15), Some(0))),
    (1, 0x1C, (Some(10), Some(7)), (Some(9), Some(7))),
    (1, 0x77, (Some(7), Some(0)), (Some(6), Some(0))),
    (1, 0xC8, (Some(1), Some(6)), (Some(1), Some(5))),
    (3, 0x05, (Some(2), Some(0)), (Some(1), Some(0))),
    (3, 0x08, (Some(2), Some(0)), (Some(1), Some(0))),
    (3, 0xFB, (None, Some(3)), (None, Some(2))),
];

#[test]
fn each_image_parses_for_its_family_and_speaks_the_mld_api() {
    for blob in [QU_B, QU_C, QUZ, CC] {
        assert!(Ucode::parse(blob).is_none(), "no image loader: not a gen3 image");
        let u = Ucode::parse_for(blob, super::rig::G2).expect("parses");
        assert!(u.iml.is_empty());
        assert_eq!(Speaks::of(&u), Ok(Speaks { cdb: false, scan: 15 }));
        assert_eq!(check(&u), Ok(()), "the join's layouts");
        assert!(!firmware_region_sizes(&classify(blob)).expect("64-page grants").is_empty());
        let fw = classify(blob);
        assert!(fw.lmac.len() <= 64 && fw.umac.len() <= 64 && fw.virt.len() <= 64);
    }
}

#[test]
fn of_the_versions_the_driver_speaks_exactly_these_differ() {
    let so = Ucode::parse(SO).unwrap();
    let all: Vec<(u8, u8)> =
        SPOKEN.iter().copied().chain(APIS.iter().map(|a| (a.group, a.cmd))).collect();
    for (name, blob) in [("Qu-b0", QU_B), ("Qu-c0", QU_C), ("QuZ", QUZ), ("cc", CC)] {
        let u = Ucode::parse_for(blob, super::rig::G2).unwrap();
        let v = |x: &Ucode<'_>, g, c| (x.cmd_version(g, c), x.notif_version(g, c));
        let mut got: Vec<(u8, u8, Vers, Vers)> = all
            .iter()
            .filter(|&&(g, c)| v(&so, g, c) != v(&u, g, c))
            .map(|&(g, c)| (g, c, v(&so, g, c), v(&u, g, c)))
            .collect();
        got.sort();
        got.dedup();
        let mut want = DIFFER.to_vec();
        if name == "cc" {
            want[7].3 = (None, None);
        }
        assert_eq!(got, want, "{name}");
    }
}
