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

//! Why an install stopped, from the installer's exit code, and whether
//! asking again could come to anything.
//!
//! The codes are the Linux personality's `install::Why`; market_proofs
//! holds the two tables equal. A reason that no retry can change (a live
//! boot with no data volume, an image built without a mirror) offers no
//! Retry: a button that can only fail again is a lie.

mod exits;
mod line;
mod model;
mod network;
mod package;
mod removal;
mod volume;

pub use exits::*;
pub use line::*;
pub use model::*;
pub use network::*;
pub use package::*;
pub use removal::*;
pub use volume::*;
