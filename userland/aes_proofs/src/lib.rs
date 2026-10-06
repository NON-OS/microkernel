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
//! Host proofs for the software AES.

pub mod hex;

/// The module tree net.nym's acknowledgement source names, so that source is
/// compiled here unchanged. Proved in `tests/nym_ack_tests.rs`.
#[cfg(test)]
mod ack;
#[cfg(test)]
mod crypto;

#[cfg(test)]
mod tests;
