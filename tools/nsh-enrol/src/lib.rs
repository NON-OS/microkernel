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

//! The fleet operator's side of nsh/1 enrolment. From a machine's
//! attestation document and host keys it makes the `nsh-enrol/1` body,
//! enrolling only values the client's own checks accept; the authority's
//! key then signs it, and the certificate is checked the way the client
//! will check it before it is written.
//!
//! What it does not do: prove that the AK lives in the machine's TPM. The
//! registrar does that through the EK (`src/security/tpm/enroll`), and
//! this tool is given the AK it activated and refuses a document quoted
//! by any other.

mod body;
mod boot;
mod challenge;
mod check;
mod encode;
mod error;
pub mod files;
pub mod hex;
mod inspect;
pub mod keys;
pub mod rand;
mod request;
mod serial;
mod show;
mod sign;

pub use body::body;
pub use boot::applications;
pub use challenge::challenge;
pub use error::Error;
pub use inspect::inspect;
pub use keys::AuthorityKey;
pub use request::Request;
pub use serial::doc_from_serial;
pub use show::{describe, describe_body};
pub use sign::{sign, verify};
