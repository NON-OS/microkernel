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

//! The kernel's signature dispatch and the verifiers it links.

// The kernel file names every algorithm; the gate reads two.
#[allow(dead_code)]
#[path = "../../../../../src/crypto/asymmetric/alg_id"]
pub mod alg_id {
    mod lengths;
    mod types;
    mod verify;

    pub use lengths::{pubkey_len, sig_len, MAX_PUBKEY_BYTES, MAX_SIG_BYTES};
    pub use types::{AlgId, AlgIdError};
    pub use verify::verify;
}

pub mod ed25519 {
    pub use nonos_ed25519::{verify, Signature};
}
