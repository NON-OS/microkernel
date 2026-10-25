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

//! Proofs for nsh_attest. `build` makes a valid document, certificate,
//! registry and policy from one `Spec`; each test changes one field of the
//! spec and shows the check refuses with exactly the name the spec gives.

#![cfg(test)]

extern crate alloc;

mod build;
/// The kernel's own document and encoder, unchanged.
#[allow(dead_code)]
#[path = "../../../src/security/attest_doc/document.rs"]
mod kernel_document;
/// The kernel's hashes, at the path its PCR 14 event builder names them.
#[allow(dead_code, deprecated, clippy::needless_range_loop)]
#[path = "../../../src/crypto"]
mod crypto {
    pub mod hash {
        pub type Hash256 = [u8; 32];
        #[path = "unified/sha1.rs"]
        mod sha1_file;
        #[path = "unified/sha256.rs"]
        mod sha256_file;
        pub mod sha384;
        pub mod sha512;
        pub use sha1_file::sha1;
        pub use sha256_file::sha256;
        pub use sha512::sha512;
    }
}
/// The kernel's PCR 14 event builder, unchanged.
#[allow(dead_code)]
#[path = "../../../src/security/capsule_measure"]
mod kernel_measure {
    pub mod algs;
    pub mod declared;
    pub mod event;
    pub mod kind;
}
mod tests;
