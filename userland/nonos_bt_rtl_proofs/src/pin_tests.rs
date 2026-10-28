// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The bytes the capsule embeds are linux-firmware's, as pinned beside it
//! (firmware/SHA256SUMS) and in the tree's upstream baseline.

use sha2::{Digest, Sha256};

use crate::firmware::{CONFIG, FW};

const SUMS: &str = include_str!("../../capsule_driver_bt_usb/firmware/SHA256SUMS");
const BASELINE: &str = include_str!("../../../scripts/baselines/upstream.sha256");
const LICENCE: &str =
    include_str!("../../capsule_driver_bt_usb/firmware/LICENCE.rtlwifi_firmware.txt");

fn hex(data: &[u8]) -> String {
    Sha256::digest(data).iter().map(|b| format!("{b:02x}")).collect()
}

fn pinned(sums: &str, name: &str) -> Option<String> {
    let line =
        sums.lines().find(|l| l.split_whitespace().nth(1).is_some_and(|f| f.ends_with(name)))?;
    Some(line.split_whitespace().next()?.to_string())
}

#[test]
fn the_embedded_files_are_the_pinned_ones() {
    for (name, data, len) in [("rtl8821c_fw.bin", FW, 54636), ("rtl8821c_config.bin", CONFIG, 10)] {
        assert_eq!(data.len(), len, "{name} is not its pinned length");
        assert_eq!(pinned(SUMS, name), Some(hex(data)), "{name} is not as SHA256SUMS pins it");
        let path = format!("userland/capsule_driver_bt_usb/firmware/{name}");
        assert_eq!(pinned(BASELINE, &path), Some(hex(data)), "{name} is not in the baseline");
    }
}

#[test]
fn a_changed_octet_is_not_the_pinned_file() {
    let mut fw = FW.to_vec();
    fw[0x5000] ^= 1;
    assert_ne!(pinned(SUMS, "rtl8821c_fw.bin"), Some(hex(&fw)));
}

#[test]
fn the_licence_travels_with_them_as_pinned() {
    assert!(LICENCE.contains("Redistribution and use in binary form, without"));
    assert_eq!(pinned(SUMS, "LICENCE.rtlwifi_firmware.txt"), Some(hex(LICENCE.as_bytes())));
}
