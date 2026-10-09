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

//! The kernel's capsule manifest and NONOS-ID certificate decoders, mounted
//! from `src/security` unchanged. Their algorithm ids resolve through the
//! same `crate::crypto::asymmetric::alg_id` path the kernel gives them; the
//! signature checks stay in the kernel and are not mounted here.

extern crate alloc;

mod alg_id;

pub mod crypto {
    pub mod asymmetric {
        pub mod alg_id {
            pub use crate::alg_id::*;
        }
    }
}

pub mod capsule_manifest;
pub mod nonos_id_cert;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod hostile_tests;
