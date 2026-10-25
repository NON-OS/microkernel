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

//! The client's checks, one function per step of nsh-v1-attest.md, each
//! refusing with its own name. `check` runs them in order.

/// The kernel's own binding, compiled from its source: the client
/// recomputes exactly what the kernel handed the TPM as `qualifyingData`.
#[path = "../../../../src/security/attest_doc/binding.rs"]
pub mod binding;
/// The registry's fold domain and entry width, from the kernel's file.
#[allow(dead_code)]
#[path = "../../../../src/security/attest_registry/format.rs"]
mod format;

mod challenge;
mod dma;
mod enrol;
mod firmware;
mod log;
mod quote;
mod registry;
mod release;

pub use challenge::challenge;
pub use dma::dma;
pub use enrol::enrol;
pub use firmware::firmware;
pub use log::log;
pub use quote::quote;
pub use registry::{fold, registry, NETWORK, VENDOR};
pub use release::release;
