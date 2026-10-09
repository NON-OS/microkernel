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

//! Host proofs for the login capsule's session, run on the real source under
//! the `crate::state` and `crate::protocol` paths its files name.

/// The capsule's status codes, which the session answers with.
#[path = "../../capsule_login/src/protocol/errno.rs"]
pub mod protocol;

/// The session state: the capsule's `state` module as it ships.
#[path = "../../capsule_login/src/state/mod.rs"]
pub mod state;

#[cfg(test)]
mod session_tests;
