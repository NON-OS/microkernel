/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* What the computer-name step holds between keys. */

use nonos_policy_proto::setup_record::HOST_MAX;

use super::rules::Refused;

/* The kernel's own name for a machine nobody named. */
const SYSTEM_HOST: &[u8] = b"nonos";

pub struct HostState {
    pub line: [u8; HOST_MAX],
    pub len: usize,
    /* The last key kept out and why, until the next key goes in. */
    pub refused: Option<(u8, Refused)>,
}

impl HostState {
    pub const fn new() -> Self {
        Self { line: [0; HOST_MAX], len: 0, refused: None }
    }

    /* What was typed; empty leaves the system's name standing. */
    pub fn typed(&self) -> &[u8] {
        &self.line[..self.len]
    }

    pub fn shown(&self) -> &[u8] {
        if self.len == 0 {
            SYSTEM_HOST
        } else {
            self.typed()
        }
    }
}
