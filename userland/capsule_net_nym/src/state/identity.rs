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

/// The client identity the gateway handshake signs with.
///
/// Generated on first use and never persisted. A fresh identity every boot means a
/// gateway cannot link two runs of the same machine, which is the property the
/// capsule exists to provide; the cost is that bandwidth credit tied to an
/// identity does not survive a reboot. A caller may still install a specific
/// identity when it needs a stable one.
static IDENTITY: Mutex<Option<Identity>> = Mutex::new(None);

/// Not `Copy`: every copy of the seed is a value whose drop wipes it, so none
/// outlives the call that asked for it.
#[derive(Clone)]
pub struct Identity {
    pub seed: [u8; 32],
    pub public: [u8; 32],
}

impl Drop for Identity {
    fn drop(&mut self) {
        crate::crypto::wipe::wipe(&mut self.seed);
    }
}

pub fn set_client_identity(seed: &[u8; 32], public: &[u8; 32]) {
    *IDENTITY.lock() = Some(Identity { seed: *seed, public: *public });
}

/// The current identity, generating one if none has been installed.
pub fn client_identity() -> Option<Identity> {
    let mut slot = IDENTITY.lock();
    if let Some(id) = slot.as_ref() {
        return Some(id.clone());
    }
    let generated = generate()?;
    *slot = Some(generated.clone());
    Some(generated)
}

fn generate() -> Option<Identity> {
    let mut id = Identity { seed: [0u8; 32], public: [0u8; 32] };
    crate::crypto::random::fill_random(&mut id.seed).ok()?;
    if crate::crypto::ed25519::pubkey(&id.seed, &mut id.public) != 32 {
        return None;
    }
    Some(id)
}
