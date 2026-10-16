// NONOS Operating System (AGPL-3.0-or-later)

//! The per-boot key and the derivation.

use nonos_mac::{apply, MAC_LEN};

use crate::wipe::wipe;
use crate::wpa::sha256::hmac_sha256_parts;

/// Bytes of key the driver draws once a boot.
pub const KEY_LEN: usize = 32;

/// What the derivation is for, so the key serves nothing else.
const LABEL: &[u8] = b"NONOS Wi-Fi station address v1";

/// The per-boot key. Not Copy; wiped when dropped.
pub struct AddressKey {
    pub(super) key: [u8; KEY_LEN],
}

impl AddressKey {
    /// Take `key`, drawn from the kernel's randomness. The caller wipes its
    /// own copy.
    pub fn new(key: &[u8; KEY_LEN]) -> Self {
        Self { key: *key }
    }

    /// The address to join `ssid` under: HMAC-SHA256 over the label and the
    /// name, its first six bytes made local and unicast.
    pub fn for_network(&self, ssid: &[u8]) -> [u8; MAC_LEN] {
        let mut h = hmac_sha256_parts(&self.key, &[LABEL, ssid]);
        let mut mac = [0u8; MAC_LEN];
        mac.copy_from_slice(&h[..MAC_LEN]);
        wipe(&mut h);
        apply(&mut mac);
        mac
    }
}

impl Drop for AddressKey {
    fn drop(&mut self) {
        wipe(&mut self.key);
    }
}
