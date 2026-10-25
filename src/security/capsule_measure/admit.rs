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

//! What a capsule's admission measures: its certificate's digest, the
//! digest of what verified it, and what that was. On a release image only a
//! STARK proof under the vendor's root is kind 1; a root a person enrolled,
//! this machine's own local-build tag, and a development image's Merkle path
//! each have their own kind, so no verifier can take one for another.

use super::kind::{kind, verifier};
use crate::crypto::hash::sha256;

/// `vendor` is whether the shipped root admitted it (`Proved::is_vendor`).
pub fn admit(cert: &[u8], trailer: &[u8], vendor: bool) {
    let local = trailer.starts_with(crate::security::local_build::MAGIC);
    let kind = kind(path_only(trailer), local, vendor);
    super::admitted(&sha256(cert), &verifier(trailer, kind), kind);
}

/// The same test the spawn gate makes before it takes a path alone
/// (`capsule_attest/path.rs`).
fn path_only(trailer: &[u8]) -> bool {
    cfg!(feature = "nonos-dev-attest") && trailer.starts_with(&nonos_attest_path::MAGIC)
}
