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

//! The host: its hybrid keys, its enrolment certificate and its kernel's
//! attestation, all from nsh_attest_proofs' default spec, which a client
//! holding that spec's policy accepts. The document is made afresh for
//! each challenge, as the kernel makes it, and quoted by the spec's AK.

use super::attest_build::{Built, Spec};
use super::roles_build::NOW;
use nsh_server::Config;

/// The host name the enrolment names.
pub const HOST_NAME: &[u8] = b"db-1.eu.example";

/// The spec every host here is built from: nsh_attest_proofs' passing
/// host, enrolled until well after the role table's clock.
pub fn spec() -> Spec {
    Spec { now_ms: NOW, cert: |c| c.not_after_ms = NOW + 30 * 86_400_000, ..Spec::default() }
}

pub fn built() -> Built {
    spec().build()
}

pub fn config() -> Config {
    Config {
        enrol: built().enrol,
        handshake_ms: 30_000,
        idle_s: 600,
        ckpt_every: 0,
        max_record: 1 << 20,
    }
}

pub use super::host_keys::Keys;
pub use super::kernel::{doc_for, Kernel};
