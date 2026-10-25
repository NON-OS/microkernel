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

//! What names the passphrase an entry was made with. Keyed by the SSID, a
//! tag would be an offline dictionary oracle for the network's password,
//! since the SSID is public. So it is keyed by 32 octets drawn from the
//! capsule's randomness when the station starts, held in a vault for this
//! boot only, never logged, never written anywhere, and gone at reboot. The
//! tag itself never leaves the driver: no reply, frame or log carries it.

use super::cache::PmksaCache;
use crate::vault::{assert_not_copy, wipe, Vault};
use crate::wpa::sha256::hmac_sha256_parts;

pub struct Tag(pub(super) Vault<32>);

assert_not_copy!(Tag);

impl Tag {
    /// HMAC-SHA256 under `key` over the SSID's length, the SSID and the
    /// passphrase; `None` for an SSID over 32 octets.
    pub(super) fn of(key: &Vault<32>, ssid: &[u8], pass: &[u8]) -> Option<Self> {
        let len = u8::try_from(ssid.len()).ok().filter(|&n| n <= 32)?;
        let mut mac = hmac_sha256_parts(key.get(), &[&[len], ssid, pass]);
        let tag = Vault::new(&mac).map(Tag);
        wipe(&mut mac);
        tag
    }

    /// Whether `other` is this tag, without stopping at the first octet
    /// that differs.
    pub(super) fn same(&self, other: &[u8; 32]) -> bool {
        let diff = self.0.get().iter().zip(other).fold(0u8, |d, (a, b)| d | (a ^ b));
        diff == 0
    }
}

impl PmksaCache {
    /// A cache whose tags are keyed by `key`, drawn fresh at each start.
    pub fn new(key: Option<Vault<32>>) -> Self {
        Self { kept: Default::default(), key }
    }

    /// The tag naming `pass` in network `ssid`, under this boot's key.
    pub fn tag(&self, ssid: &[u8], pass: &[u8]) -> Option<Tag> {
        Tag::of(self.key.as_ref()?, ssid, pass)
    }
}
