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

/// nonos_aes's S-box and its inverse, compiled in from its own source at the
/// paths that source names them by, so the inverse is checked as it ships.
#[cfg(test)]
#[path = "../../nonos_aes/src/inv_sub_byte.rs"]
mod inv_sub_byte;
#[cfg(test)]
#[path = "../../nonos_aes/src/sub_byte.rs"]
mod sub_byte;
#[cfg(test)]
#[path = "../../nonos_aes/src/xtime.rs"]
mod xtime;

#[cfg(test)]
mod tests;
