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

//! Services whose display size hints the compositor takes (NCMP 0x0009):
//! none until the owner accepts display hints (Decision 15). Data only,
//! kept beside the display list so the owner rules on both in one place.
//! The compositor mounts this file by path; the kernel has no use for it
//! and does not build it.

pub(crate) const HINT_SOURCES: &[&str] = &[];
