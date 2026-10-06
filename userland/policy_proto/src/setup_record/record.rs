/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The record setup writes now: its answers, the apps it turned off, and
 * the computer's name.
 */

use super::answers::Answers;
use super::kept::Host;
use super::layout::{ANSWERS_LEN, ANSWERS_V2_LEN, ANSWERS_V3_LEN, ANSWERS_V4_LEN, ANSWERS_V5_LEN};
use super::layout::{APPS_AT, HOST_AT, MAGIC_V5, MAGIC_V6, ROUTE_AT, WALLPAPERS_AT};
use super::layout::{MAGIC_V3, MAGIC_V4};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Record {
    pub answers: Answers,
    /* The apps turned off, as crate::apps has them. Zero in version 1 and 2. */
    pub apps_off: u8,
    /* Empty before version 4, and when no computer name was given. */
    pub hostname: Host,
    /* The network the system's own traffic leaves through, as crate::route
     * has it: the Nym mixnet in a record older than version 5. */
    pub route: u8,
    /* The wallpapers kept, as crate::wallpapers_kept has them: every one in
     * a record older than version 6. */
    pub wallpapers_kept: u64,
}

impl Record {
    pub fn encode(&self) -> [u8; ANSWERS_LEN] {
        let mut out = [0u8; ANSWERS_LEN];
        out[..ANSWERS_V5_LEN].copy_from_slice(&self.encode_v5());
        out[..4].copy_from_slice(&MAGIC_V6);
        out[WALLPAPERS_AT..].copy_from_slice(&self.wallpapers_kept.to_le_bytes());
        out
    }

    /* Version 5, for a store that holds one: everything but the wallpapers
     * kept. */
    pub fn encode_v5(&self) -> [u8; ANSWERS_V5_LEN] {
        let mut out = [0u8; ANSWERS_V5_LEN];
        out[..ANSWERS_V4_LEN].copy_from_slice(&self.encode_v4());
        out[..4].copy_from_slice(&MAGIC_V5);
        out[ROUTE_AT] = self.route;
        out
    }

    /* Version 4, for a store that holds one: everything but the route. */
    pub fn encode_v4(&self) -> [u8; ANSWERS_V4_LEN] {
        let mut out = [0u8; ANSWERS_V4_LEN];
        out[..ANSWERS_V3_LEN].copy_from_slice(&self.encode_v3());
        out[..4].copy_from_slice(&MAGIC_V4);
        self.hostname.put(&mut out[HOST_AT..]);
        out
    }

    /* Version 3, for a store that holds one: everything but the name. */
    pub fn encode_v3(&self) -> [u8; ANSWERS_V3_LEN] {
        let mut out = [0u8; ANSWERS_V3_LEN];
        out[..ANSWERS_V2_LEN].copy_from_slice(&self.answers.encode());
        out[..4].copy_from_slice(&MAGIC_V3);
        out[APPS_AT] = self.apps_off;
        out
    }

    /* A record of any version this reads, or `None`; see `check_record`. */
    pub fn decode(raw: &[u8]) -> Option<Record> {
        super::check::check_record(raw).ok()
    }
}
