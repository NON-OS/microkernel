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

//! Reading the network's own description of itself.
//!
//! Two documents, fetched from the authorities. The consensus says which relays
//! exist, where they are and what they are trusted for; the microdescriptors it
//! points at carry the keys a circuit is actually built with. Neither half is
//! usable alone.

pub mod authority;
mod base64;
mod lines;
mod number;
mod time;

pub mod consensus;
pub mod fetch;
pub mod microdesc;
pub mod verify;
