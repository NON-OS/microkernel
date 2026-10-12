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

//! NONOS's SMBIOS reader, written from DMTF DSP0134 3.x alone: the 32- and
//! 64-bit entry points, and a walk of the structure table that checks
//! every length before it reads, so a table firmware wrote wrong ends the
//! walk instead of reading past it. Pure: it is handed bytes and does no
//! I/O of its own.

#![no_std]

extern crate alloc;

pub mod chassis;
pub mod entry;
pub mod identity;
pub mod quirk;
pub mod structure;
pub mod text;
pub mod walk;
