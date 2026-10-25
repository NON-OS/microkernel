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

// The rest of `security`, included into it: the key store, the memory
// sanitizer's canary and erase passes (the scrub routine stands in), and
// the quantum engine's generator and key vault, whose proofs sit inside
// it. Its key generation stands in.

#[path = "../../../src/security/crypto"]
pub mod crypto {
    pub mod key_management;
}

#[path = "../../../src/security/hardening"]
pub mod hardening {
    pub mod memory_sanitization {
        pub mod canary;
        pub mod erase;
        mod primitives;
        pub mod state;
        pub mod types;
        pub use crate::stand_in::scrub_asm;
    }
}

#[path = "../../../src/security/quantum"]
pub mod quantum {
    pub mod pqc {
        pub mod rng;
        pub mod types;
        pub use crate::stand_in::pqc::pq_ops;
        pub mod vault {
            include!("../../../src/security/quantum/pqc/vault.rs");
            #[warn(clippy::all)]
            mod tests {
                include!("vault_tests.rs");
            }
        }
    }
}
