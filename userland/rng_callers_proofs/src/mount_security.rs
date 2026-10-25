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

// The kernel's src/security, included into the crate root: the session
// accounts, the advanced policy's canary and ASLR, and the capability
// engine. The keyring, the TPM key's error and the audit log stand in. The
// engine's and the canary's proofs sit inside them, where their private
// fields are in reach; the canary's file is the build script's copy.
#[allow(clippy::all)]
#[path = "../../../src/security"]
pub mod security {
    pub use crate::stand_in::{audit, keyring_capsule};
    pub mod tpm {
        pub use crate::stand_in::machine_key;
    }

    pub mod policy {
        pub mod session;
        pub mod advanced {
            pub mod canary {
                include!(concat!(env!("OUT_DIR"), "/advanced_canary.rs"));
                #[warn(clippy::all)]
                mod tests {
                    include!("canary_tests.rs");
                }
            }
            pub mod cfi;
            pub mod config;
            pub mod flags;
            pub mod global;
            pub mod manager;
            pub use {canary::*, cfi::*, config::*, flags::*, global::*, manager::*};
        }
        pub mod capability {
            pub mod attestation;
            pub mod isolation;
            pub mod quantum;
            pub mod stats;
            pub mod types;
            pub mod violations;
            // Declared from lib.rs (`engine_mount`), so its tests resolve from
            // src/ for rustc and the lane check alike.
            pub use crate::engine_mount as engine;
        }
    }

    include!("mount_security_more.rs");
}
