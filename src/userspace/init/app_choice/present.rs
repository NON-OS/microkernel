/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* The switches whose apps this kernel carries, from the capsules built in. */

use super::bits::{BROWSER, CALCULATOR, EDITOR, FILES, LINUX, MEDIA, STORE, WALLET};

const fn has(built_in: bool, bit: u32) -> u32 {
    if built_in {
        bit
    } else {
        0
    }
}

const MEDIA_BUILT_IN: bool = cfg!(feature = "nonos-capsule-audio-player")
    || cfg!(feature = "nonos-capsule-video-player")
    || cfg!(feature = "nonos-capsule-image-viewer");

pub(crate) const PRESENT: u32 = has(cfg!(feature = "nonos-capsule-browser"), BROWSER)
    | has(cfg!(feature = "nonos-capsule-wallet-nonos"), WALLET)
    | has(cfg!(feature = "nonos-capsule-app-store"), STORE)
    | has(cfg!(feature = "nonos-capsule-file-manager"), FILES)
    | has(cfg!(feature = "nonos-capsule-text-editor"), EDITOR)
    | has(cfg!(feature = "nonos-capsule-calculator"), CALCULATOR)
    | has(MEDIA_BUILT_IN, MEDIA)
    | has(cfg!(feature = "nonos-capsule-linux"), LINUX);
