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

//! Proofs for nsh_server. `kit` holds the host the proofs run: its keys,
//! its enrolment and its attestation, built by nsh_attest_proofs' builders;
//! its role table and revocation list, built by nsh_roles_proofs'; the user
//! and the client policy; a raw client for frames the real one never
//! sends; and the pump that runs a client against a server in memory.

pub mod kit;
#[cfg(test)]
mod tests;
