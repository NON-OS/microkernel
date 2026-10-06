/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 1, for a store that holds one. The store replaces a record only
 * with one of the same length, so what goes where an earlier build left a
 * record is that record's version, holding what it can.
 */

use super::answers::Answers;
use super::layout::{ANSWERS_V1_LEN, MAGIC_V1};

impl Answers {
    /* Version 1: keyboard, time zone and wallpaper only. */
    pub fn encode_v1(&self) -> [u8; ANSWERS_V1_LEN] {
        let mut out = [0u8; ANSWERS_V1_LEN];
        out[..4].copy_from_slice(&MAGIC_V1);
        out[4] = self.keyboard_layout;
        out[5] = self.timezone as u8;
        out[6] = self.wallpaper;
        out
    }
}
