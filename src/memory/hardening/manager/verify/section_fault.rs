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

//! What is wrong with a kernel section's mapping, in enough detail to act on.
//!
//! A count of conforming sections says a boot is not clean and nothing more.
//! Naming the page and what the hardware grants there is the difference
//! between a second forty minute boot and a fix.

/// The first page of a section that does not match its descriptor.
#[derive(Clone, Copy)]
pub struct SectionFault {
    pub va: u64,
    /// None when no page-table entry maps `va` at all.
    pub granted: Option<Granted>,
    pub want_writable: bool,
    pub want_executable: bool,
}

#[derive(Clone, Copy)]
pub struct Granted {
    pub writable: bool,
    pub executable: bool,
}
