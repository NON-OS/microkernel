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

//! The per-boot secret, and the addresses made from it.

use super::bits::{identity, local_unicast};
use crate::vault::{assert_not_copy, Vault};
use crate::wpa::sha256::hmac_sha256_parts;

pub struct Addresses {
    /// 32 octets from the system's randomness, drawn once per boot and
    /// never kept anywhere else.
    key: Vault<32>,
    /// The last scan's address, so the next scan never reuses it.
    pub(super) last_scan: Option<[u8; 6]>,
}

assert_not_copy!(Addresses);

impl Addresses {
    /// The secret drawn by `fill`; `None` when there is no randomness.
    pub fn new(fill: impl FnOnce(&mut [u8; 32]) -> bool) -> Option<Self> {
        Some(Self { key: Vault::fill(fill)?, last_scan: None })
    }

    /// The station's secret: drawn fresh at each start, so no network sees
    /// this boot's address again after a reboot. This is the one place the
    /// secret comes from; owner Decision 14, on whether a network may keep
    /// its address across reboots, changes only this.
    pub fn for_this_boot(random: impl FnOnce(&mut [u8]) -> bool) -> Option<Self> {
        Self::new(|k| random(k))
    }

    /// The address for network `ssid` under `security` (`security_of`).
    pub fn join(&self, ssid: &[u8], security: u8) -> [u8; 6] {
        let mac = hmac_sha256_parts(self.key.get(), &identity(ssid, &security));
        let mut a = [0u8; 6];
        a.copy_from_slice(&mac[..6]);
        local_unicast(a)
    }
}
