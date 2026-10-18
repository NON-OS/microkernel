// NONOS Operating System (AGPL-3.0-or-later)

//! The bundled images describe themselves.

use crate::gen3::ucode::Ucode;

pub(super) static SO_GF: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

static SO_HR: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-hr-b0-84.ucode");

#[test]
fn the_bundled_ax211_image_describes_itself() {
    let u = Ucode::parse(SO_GF).expect("parses");
    assert_eq!(u.iml.len(), 13944, "the image loader");
    assert_eq!(u.phy_config, 0x0033_0018);
    assert_eq!((u.valid_tx_ant(), u.valid_rx_ant()), (3, 3), "antennas A and B");
    assert_eq!(u.n_scan_channels, 67);
    assert!(u.capa(110), "MLD API");
    assert!(u.capa(1) && u.capa(2), "LAR and UMAC scan");
    assert!(!u.capa(12), "no DQA command");
    assert!(u.api(48), "NVM reply version 4");
    assert_eq!(u.cmd_version(1, 0x0D), Some(17), "SCAN_REQ_UMAC version 17");
    assert_eq!(u.cmd_version(0, 0x0D), Some(17), "legacy ids are looked up in LONG_GROUP");
    assert_eq!(u.notif_version(0, 0x01), Some(6), "ALIVE notification version 6");
    assert_eq!(u.notif_version(1, 0xC8), Some(6), "MCC reply version 6");
    assert_eq!(u.cmd_version(1, 0x28), None, "MAC_CONTEXT_CMD is not listed");
    assert_eq!(u.cmd_version(0, 0x01), None, "a version 99 entry reads as not reported");
}

#[test]
fn the_bundled_ax201_image_parses_too() {
    let u = Ucode::parse(SO_HR).expect("parses");
    assert!(!u.iml.is_empty());
    assert_eq!(u.notif_version(1, 0xC8), Some(5));
    assert_eq!(u.cmd_version(1, 0x0D), Some(17));
}
