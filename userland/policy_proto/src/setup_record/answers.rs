// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/*
 * The answers setup keeps, and the bytes it writes for them.
 */

use super::kept::{Name, Tier};
use super::layout::{ANSWERS_LEN, ANSWERS_V1_LEN, MAGIC_V1, MAGIC_V2, NAME_AT, TIER_AT};

/* The policy fields setup restores on a later boot. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Answers {
    pub keyboard_layout: u8,
    pub timezone: i8,
    pub wallpaper: u8,
    /* Empty in a version 1 record, and when no name was given. */
    pub username: Name,
    /* Empty in a version 1 record, and when no tier was chosen. */
    pub qwen_tier: Tier,
}

impl Answers {
    /* The current version, the one setup writes. */
    pub fn encode(&self) -> [u8; ANSWERS_LEN] {
        let mut out = [0u8; ANSWERS_LEN];
        out[..ANSWERS_V1_LEN].copy_from_slice(&self.encode_v1());
        out[..4].copy_from_slice(&MAGIC_V2);
        self.username.put(&mut out[NAME_AT..TIER_AT]);
        self.qwen_tier.put(&mut out[TIER_AT..]);
        out
    }

    /*
     * Version 1, without the name and the tier. The store replaces a record
     * only with one of the same length, so this is what may still go where
     * an earlier build left a version 1 record.
     */
    pub fn encode_v1(&self) -> [u8; ANSWERS_V1_LEN] {
        let mut out = [0u8; ANSWERS_V1_LEN];
        out[..4].copy_from_slice(&MAGIC_V1);
        out[4] = self.keyboard_layout;
        out[5] = self.timezone as u8;
        out[6] = self.wallpaper;
        out
    }

    /*
     * `None` for anything but a record `encode` or `encode_v1` wrote.
     * Zeros, which is how the store withdraws a record, have no magic and
     * so read as absent. `check` says why a record was refused.
     */
    pub fn decode(raw: &[u8]) -> Option<Answers> {
        super::check::check(raw).ok()
    }
}
