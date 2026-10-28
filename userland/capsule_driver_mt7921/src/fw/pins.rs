/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The six files of the MediaTek firmware package this capsule accepts,
//! by slot: name, length and SHA-256 of linux-firmware's files at commit
//! bf992df7a95b5e19bb8f8546eb4d7c53ab2598d0 (WHENCE: "Licence:
//! Redistributable. See LICENCE.mediatek"). Only these numbers are in the
//! tree; the files are not. `scripts/check_mediatek_firmware.py` reads
//! this table and holds the loader's, the kernel's and the build's to it.

pub const PINS: [(&str, usize, &str); 6] = [
    (
        "WIFI_MT7961_patch_mcu_1_2_hdr.bin",
        92192,
        "a276c06c2b772adb50b86639d33c82824ff4c21d617feb78caea74c040b873f6",
    ),
    (
        "WIFI_RAM_CODE_MT7961_1.bin",
        792036,
        "b94217a951518a9c14095765f367bc5dd7698f2dc033941d6f18fc2ebd6a2ab9",
    ),
    (
        "WIFI_MT7961_patch_mcu_1a_2_hdr.bin",
        100384,
        "a766c4e39815e554c80ce5b1596095ff283a60db93356679fecd6494f71cee23",
    ),
    (
        "WIFI_RAM_CODE_MT7961_1a.bin",
        802808,
        "3edec74ffe341b917e7fa4ddb2b68851411f2416555f10619a5900fa4b141e6c",
    ),
    (
        "WIFI_MT7922_patch_mcu_1_1_hdr.bin",
        137632,
        "7fc9075d6d54b31e0539734614ca971fa52caf41b642a3000c8a4501591a4d98",
    ),
    (
        "WIFI_RAM_CODE_MT7922_1.bin",
        1231252,
        "d100e5e04c324fc8da6302112d1e0b4fac952286447d2419eabcd8fb0a1f5b93",
    ),
];

/// A pinned hash as bytes, at compile time.
pub const fn unhex(s: &str) -> [u8; 32] {
    let b = s.as_bytes();
    assert!(b.len() == 64);
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        out[i] = nib(b[2 * i]) << 4 | nib(b[2 * i + 1]);
        i += 1;
    }
    out
}

const fn nib(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => panic!("a pinned hash is lowercase hex"),
    }
}
