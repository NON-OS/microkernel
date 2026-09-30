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

use super::mint::mint;

pub struct LocalIdentity {
    pub secret: [u8; 32],
    pub blinding: [u8; 32],
    pub commitment: [u8; 32],
    pub root: [u8; 32],
    /// Derived from the machine key, so the same on every boot of this
    /// machine running this kernel. False when there is no TPM to ask.
    pub persistent: bool,
}

static IDENTITY: Mutex<Option<LocalIdentity>> = Mutex::new(None);

/// The root to enrol so this machine will run what it builds.
pub fn root() -> Option<[u8; 32]> {
    with_identity(|id| id.root)
}

/// Whether consent to this identity can outlive the boot.
pub(super) fn persistent() -> bool {
    with_identity(|id| id.persistent).unwrap_or(false)
}

/// Run `f` on the identity, minting it first if this boot has none.
///
/// The lock is held across minting and across a whole proof, and it is
/// reached from system calls with interrupts masked, so a CPU waiting for
/// it answers TLB shootdowns while it spins.
pub(super) fn with_identity<T>(f: impl FnOnce(&LocalIdentity) -> T) -> Option<T> {
    let mut guard = crate::smp::lock_responsive(&IDENTITY);
    if guard.is_none() {
        *guard = Some(mint()?);
    }
    guard.as_ref().map(f)
}
