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

/*
 * The wallet's side of Shield. The shield wallet itself, the phones' own
 * shield-core with its prover, runs as the nonos.shield service; this is
 * the client that opens it for this account, asks it for reviews and
 * proofs, and follows what it reports.
 */

pub mod actions;
mod apply;
pub mod client;
mod follow;
pub mod job;
mod kept_names;
pub mod open;
pub mod probe;
pub mod reply;

pub use follow::due;
