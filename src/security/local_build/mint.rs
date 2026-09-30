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

//! Minting this machine's local-build identity.

use crate::crypto::rng::get_random_bytes_secure;
use crate::crypto::zk_kernel::PedersenCommitment;
use crate::security::tpm::machine_key::derive_for_kernel;

use super::identity::LocalIdentity;
use super::tree::root_for;

/*
 * The machine key when there is one, so a person consents once per machine.
 * Without a TPM a random identity for this boot is the honest fallback, never
 * a fixed one: a guessable secret is a tree anyone can mint proofs against.
 */
pub(super) fn mint() -> Option<LocalIdentity> {
    let (secret, blinding, persistent) = match (
        derive_for_kernel(b"local_build/secret"),
        derive_for_kernel(b"local_build/blinding"),
    ) {
        (Ok(s), Ok(b)) => (s, b, true),
        _ => {
            crate::sys::serial::println(b"[LOCAL-BUILD] no machine key; identity lasts this boot");
            (get_random_bytes_secure().ok()?, get_random_bytes_secure().ok()?, false)
        }
    };
    let commitment = PedersenCommitment::commit(&secret, &blinding).commitment;
    let root = root_for(&commitment);
    Some(LocalIdentity { secret, blinding, commitment, root, persistent })
}
