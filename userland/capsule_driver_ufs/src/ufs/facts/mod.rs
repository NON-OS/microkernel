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

//! The fact sheet: every register offset, bit, code, layout and bound the
//! engine uses, one module per block, each fact with where it comes from.
//! None of the documents could be opened while this was written, so every
//! fact is marked unverified until someone checks it against them.

pub mod bounds;
pub mod hci;
pub mod uic;
pub mod upiu;
pub mod utp;
