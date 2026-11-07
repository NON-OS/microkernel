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

//! The host, the user and the runs the proofs share.

/// nsh_attest_proofs' builders: a P-256 AK, TCG logs, quotes, `NONOSATT`
/// version 5 documents and enrolment certificates.
#[path = "../../../nsh_attest_proofs/src/build/mod.rs"]
pub mod attest_build;
/// nsh_roles_proofs' builders: role tables, revocation lists and grants.
#[path = "../../../nsh_roles_proofs/src/build/mod.rs"]
pub mod roles_build;

pub mod demo;
pub mod devkit;
pub mod host;
pub mod host_keys;
pub mod kernel;
pub mod noise;
pub mod noise_check;
pub mod noise_plain;
pub mod raw;
pub mod raw_auth;
pub mod raw_frames;
pub mod raw_run;
pub mod raw_session;
pub mod rng;
pub mod roles;
pub mod run;
pub mod run_pump;
pub mod run_read;
pub mod run_start;
pub mod user;
