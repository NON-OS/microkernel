// NONOS Operating System (AGPL-3.0-or-later)

//! The download mode words and announcements: `mt76_connac2_get_data_mode`
//! for patch sections (plain, AES with its key index, scrambled, not
//! supported), `mt76_connac_mcu_gen_dl_mode` for RAM regions, and
//! `mt76_connac_mcu_init_download`'s choice of command by address.

use crate::fw::mode::{patch_mode, ram_mode, request_for};
use crate::mcu::cmd::{PATCH_START_REQ, TARGET_ADDRESS_LEN_REQ};

const NEED_RSP: u32 = 1 << 31;

#[test]
fn a_patch_sections_mode_follows_its_encryption() {
    assert_eq!(patch_mode(0), NEED_RSP, "plain (the MT7961 files)");
    assert_eq!(patch_mode(u32::MAX), NEED_RSP, "PATCH_SEC_NOT_SUPPORT");
    assert_eq!(patch_mode(0x0100_0000), NEED_RSP | 0x1 | 0x8, "AES, key 0 (the MT7922 file)");
    assert_eq!(patch_mode(0x0100_0003), NEED_RSP | 0x1 | 0x6 | 0x8, "AES, key 3");
    assert_eq!(patch_mode(0x0100_0004), NEED_RSP | 0x1 | 0x8, "the key index is two bits");
    assert_eq!(patch_mode(0x0200_0000), NEED_RSP | 0x1 | 0x40 | 0x8, "scrambled");
    assert_eq!(patch_mode(0x0700_0000), NEED_RSP, "unknown: sent plain, as Linux");
}

#[test]
fn a_ram_regions_mode_follows_its_feature_byte() {
    assert_eq!(ram_mode(0x00), NEED_RSP);
    assert_eq!(ram_mode(0x21), NEED_RSP | 0x1 | 0x8, "encrypted, the override bit ignored");
    assert_eq!(ram_mode(0x01 | 0x6), NEED_RSP | 0x1 | 0x6 | 0x8, "key index 3");
    assert_eq!(ram_mode(0x10), NEED_RSP | 0x40, "FW_FEATURE_ENCRY_MODE");
    assert_eq!(ram_mode(0x40), NEED_RSP, "a region not downloaded has no mode bits");
}

#[test]
fn the_patch_area_is_announced_by_patch_start_and_the_rest_by_address() {
    assert_eq!(request_for(0x900000), PATCH_START_REQ);
    assert_eq!(request_for(0x200000), PATCH_START_REQ);
    assert_eq!(request_for(0x91b000), TARGET_ADDRESS_LEN_REQ);
    assert_eq!(request_for(0x900001), TARGET_ADDRESS_LEN_REQ);
}
