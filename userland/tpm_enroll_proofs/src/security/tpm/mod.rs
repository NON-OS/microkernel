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

//! The kernel's TPM tree as far as enrollment reaches into it, with the
//! transport swapped for swtpm's TCP command port.

pub mod ak;
#[path = "../../../../../src/security/tpm/device_secret/mod.rs"]
pub mod device_secret;
#[path = "../../../../../src/security/tpm/enroll/mod.rs"]
pub mod enroll;
#[path = "../../../../../src/security/tpm/error.rs"]
pub mod error;
pub mod machine_key;
#[path = "../../../../../src/security/tpm/quote/mod.rs"]
pub mod quote;
#[path = "../../../../../src/security/tpm/resend.rs"]
mod resend;
mod transport;

pub use resend::transact_resending;
pub use transport::{transact, PORT};

#[cfg(test)]
pub(crate) mod live;
#[cfg(test)]
mod parse_hash_tests;
#[cfg(test)]
mod parse_nv_tests;
#[cfg(test)]
mod parse_sign_tests;
#[cfg(test)]
mod parse_tests;
#[cfg(test)]
mod parse_util;
