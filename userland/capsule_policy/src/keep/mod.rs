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

//! Keeping what Settings changes, on a machine that keeps state.
//!
//! The policy store is memory. Setup's answers come back at boot from the
//! record setup kept; anything changed after that would not, and an
//! installed machine would forget every change made in Settings. So when a
//! value changes on a machine that keeps state (Persistent), the kept fields
//! (settings_record::KEPT) are written to the settings record once the
//! changes have been quiet for a moment, and restore puts them back after the
//! answers. An amnesic boot keeps nothing, so writes nothing.

mod snapshot;
mod tick;

pub use tick::{tick, written_now};
