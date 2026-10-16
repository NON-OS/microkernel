// NONOS Operating System (AGPL-3.0-or-later)

//! A passphrase on its way to the driver, wiped when dropped.

use nonos_wifi_client::wipe;

use super::super::cache::STRING_CAP;

/// A passphrase on its way to the driver, wiped when dropped.
pub(in super::super) struct Secret {
    bytes: [u8; STRING_CAP],
    len: usize,
}

impl Secret {
    pub(in super::super) fn new(from: &[u8]) -> Self {
        let mut s = Self { bytes: [0u8; STRING_CAP], len: from.len().min(STRING_CAP) };
        s.bytes[..s.len].copy_from_slice(&from[..s.len]);
        s
    }

    pub(super) fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        wipe(&mut self.bytes);
        self.len = 0;
    }
}
