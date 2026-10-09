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

use spin::Mutex;

use crate::crypto::rng::get_random_bytes_secure;
use crate::security::tpm::machine_key::derive_for_kernel;

/* Names the key in public. One-way, so the root says whose key without giving it. */
const ROOT_CONTEXT: &str = "NONOS local build root v1";

pub(super) struct LocalIdentity {
    pub key: [u8; 32],
    pub root: [u8; 32],
    /// Derived from the machine key, so the same on every boot of this
    /// machine running this kernel. False when there is no TPM to ask.
    pub persistent: bool,
}

static IDENTITY: Mutex<Option<LocalIdentity>> = Mutex::new(None);

/*
 * The machine key when there is one, so a person consents once per machine.
 * Without a TPM a random key for this boot is the honest fallback, never a
 * fixed one: a guessable key is one anyone can tag builds under.
 */
fn mint() -> Option<LocalIdentity> {
    let (key, persistent) = match derive_for_kernel(b"local_build/key") {
        Ok(k) => (k, true),
        Err(_) => {
            crate::sys::serial::println(b"[LOCAL-BUILD] no machine key; identity lasts this boot");
            (get_random_bytes_secure().ok()?, false)
        }
    };
    let root = blake3::derive_key(ROOT_CONTEXT, &key);
    Some(LocalIdentity { key, root, persistent })
}

/// The root to enrol so this machine will run what it builds.
pub fn root() -> Option<[u8; 32]> {
    with_identity(|id| id.root)
}

/// Whether consent to this identity can outlive the boot.
pub(super) fn persistent() -> bool {
    with_identity(|id| id.persistent).unwrap_or(false)
}

pub(super) fn with_identity<T>(f: impl FnOnce(&LocalIdentity) -> T) -> Option<T> {
    /*
     * Reached from system calls with interrupts masked, so a CPU waiting here
     * answers TLB shootdowns while it spins.
     */
    let mut guard = crate::smp::lock_responsive(&IDENTITY);
    if guard.is_none() {
        *guard = Some(mint()?);
    }
    guard.as_ref().map(f)
}
