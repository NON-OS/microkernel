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

//! The record that a person let this machine run what it installs.
//!
//! A token is an HMAC over the local root under a machine key only the kernel
//! can derive. It is worthless on another machine or under another kernel,
//! and it cannot be made from the disk it is kept on, so keeping it anywhere is
//! safe. Presenting it restores a consent that was already given; it cannot
//! give one.

use crate::crypto::hash::hmac_sha256;
use crate::security::tpm::machine_key::derive_for_kernel;

/// The token for `root`, or `None` when this machine cannot keep consent.
pub fn token(root: &[u8; 32]) -> Option<[u8; 32]> {
    if !super::identity::persistent() {
        return None;
    }
    let key = derive_for_kernel(b"local_build/consent").ok()?;
    Some(hmac_sha256(&key, root))
}
