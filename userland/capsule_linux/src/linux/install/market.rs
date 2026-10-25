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
//! A market program installed in the box its release names: the
//! fourth word of the request (arg), what the policy is held to (check),
//! the install (run) and its binding (bind), and what keeps the package's
//! programs from starting unboxed meanwhile (minted, take, hold).

pub(super) mod arg;
mod bind;
mod check;
pub(super) mod hold;
pub(super) mod minted;
mod run;
pub(super) mod take;

pub use run::install_from;
