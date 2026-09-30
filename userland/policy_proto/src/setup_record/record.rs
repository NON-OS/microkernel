/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The record setup writes now: its answers, and the apps it turned off.
 */

use super::answers::Answers;
use super::layout::{ANSWERS_LEN, ANSWERS_V2_LEN, APPS_AT, MAGIC_V3};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Record {
    pub answers: Answers,
    /* The apps turned off, as crate::apps has them. Zero in version 1 and 2. */
    pub apps_off: u8,
}

impl Record {
    pub fn encode(&self) -> [u8; ANSWERS_LEN] {
        let mut out = [0u8; ANSWERS_LEN];
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
