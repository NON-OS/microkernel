/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The network a save dropped to make room, named for the screen only.
//!
//! It carries the name and nothing else: no passphrase, no flags. The list
//! keeps no record of it; the panel that asked for the save shows the name
//! once, and no log ever carries it.

use crate::network::SSID_MAX;

#[derive(Clone, Copy)]
pub struct Dropped {
    ssid: [u8; SSID_MAX],
    len: u8,
}

impl Dropped {
    pub(super) fn of(ssid: &[u8]) -> Self {
        let mut d = Self { ssid: [0; SSID_MAX], len: ssid.len().min(SSID_MAX) as u8 };
        d.ssid[..d.len as usize].copy_from_slice(&ssid[..d.len as usize]);
        d
    }

    /// The dropped network's name.
    pub fn ssid(&self) -> &[u8] {
        &self.ssid[..self.len as usize]
    }
}
