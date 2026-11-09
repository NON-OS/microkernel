/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The two Bluetooth files of the MediaTek firmware package this capsule
//! accepts, by slot: name, length and SHA-256 of linux-firmware's files at
//! commit bf992df7a95b5e19bb8f8546eb4d7c53ab2598d0, the Wi-Fi driver's
//! (WHENCE: "Licence: Redistributable. See LICENCE.mediatek"). Only these
//! numbers are in the tree; the files are not. They follow the Wi-Fi
//! driver's six in the package, and `scripts/check_mediatek_firmware.py`
//! holds the loader's, the kernel's and the build's lists to both tables.

pub const PINS: [(&str, usize, &str); 2] = [
    (
        "BT_RAM_CODE_MT7961_1_2_hdr.bin",
        533870,
        "7a440a0f7404c73a2f03d8cc0475ac9f7fdc931b494dce37705e7d578153dbae",
    ),
    (
        "BT_RAM_CODE_MT7922_1_1_hdr.bin",
        517398,
        "299942dbec7d34af030670ea59f9f7b4fd5c64e738b9e0157d76f4dd3beb190f",
    ),
];

/// A pinned hash as octets, at compile time.
pub const fn hash(hex: &str) -> [u8; 32] {
    let h = hex.as_bytes();
    assert!(h.len() == 64, "a pinned hash is 64 hex digits");
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 64 {
        out[i / 2] = out[i / 2] << 4 | digit(h[i]);
        i += 1;
    }
    out
}

const fn digit(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => panic!("a pinned hash is lowercase hex"),
    }
}
