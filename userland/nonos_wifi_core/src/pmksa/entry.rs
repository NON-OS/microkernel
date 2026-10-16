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

//! One PMK security association (IEEE Std 802.11-2020, 12.6.10.3): the PMK
//! an SAE exchange with one AP produced, and the PMKID that names it. With
//! one cached, a later join to the same AP may skip SAE: Open System
//! authentication, then a (Re)Association Request whose RSNE carries the
//! PMKID, and the four-way handshake under the cached PMK.

/// The PMK and the PMKID SAE gave for one AP.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pmksa {
    pub bssid: [u8; 6],
    pub pmk: [u8; 32],
    pub pmkid: [u8; 16],
}

/// One entry as the cache keeps it.
#[derive(Clone, Copy)]
pub(crate) struct Kept {
    pub(crate) ssid: [u8; 32],
    pub(crate) ssid_len: usize,
    pub(crate) since_ms: u64,
    /// HMAC-SHA256 keyed by the SSID over the passphrase.
    pub(crate) tag: [u8; 32],
    pub(crate) sa: Pmksa,
}

/// Overwrite an entry's keys and tag with volatile writes, so the compiler
/// cannot drop the stores.
pub(crate) fn wipe(p: &mut Pmksa, tag: &mut [u8; 32]) {
    for b in p.pmk.iter_mut().chain(p.pmkid.iter_mut()).chain(tag.iter_mut()) {
        // SAFETY: `b` is a valid, aligned, exclusive reference into `p`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
