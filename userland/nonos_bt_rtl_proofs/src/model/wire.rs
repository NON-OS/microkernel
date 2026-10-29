// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The model's answers as they go on the wire.

use super::Rtl;

/// Command Complete (Vol 4, Part E, 7.7.14), one command taken.
pub(super) fn complete(op: u16, status: u8, ret: &[u8]) -> Vec<u8> {
    let [o0, o1] = op.to_le_bytes();
    [&[0x0e, 4 + ret.len() as u8, 1, o0, o1, status][..], ret].concat()
}

impl Rtl {
    /// Read Local Version (Vol 4, Part E, 7.4.1): HCI version 8, then the
    /// ROM's revision and subversion, or the running patch's, as btrtl.c
    /// prints them after a download ("fw version" from the two).
    pub(super) fn version(&self) -> Vec<u8> {
        let (rev, sub) = match self.running {
            Some(v) => ((v >> 16) as u16, v as u16),
            None => (0x000c, 0x8821),
        };
        let ([r0, r1], [s0, s1]) = (rev.to_le_bytes(), sub.to_le_bytes());
        vec![0x08, r0, r1, 0x08, 0x5d, 0x00, s0, s1]
    }

    /// The version the image carries: the four octets before the config.
    pub(super) fn carried(&self, config_len: usize) -> u32 {
        let at = self.got.len() - config_len - 4;
        u32::from_le_bytes(self.got[at..at + 4].try_into().expect("four octets"))
    }
}
