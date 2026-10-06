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

//! Reading the microdescriptor flavour consensus.

mod build;
mod document;
mod entry;
mod flags;
mod pem;
mod router;
mod scan;
mod signature;
pub mod span;
mod weights;

pub use document::{is_stale, Consensus};
pub use entry::Entry;
pub use pem::object_after;
pub use scan::parse;
pub use signature::Signature;
