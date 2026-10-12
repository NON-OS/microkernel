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

//! NONOS's own AML interpreter, written from the ACPI 6.5 specification alone.
//!
//! On most systems AML runs in ring 0, where one firmware bug or one hostile
//! SSDT reaches all of memory. Here it runs in `capsule_acpi`, holding only the
//! OperationRegion windows the broker grants, so a method that loops or faults
//! stops the capsule and not the machine. This crate is the pure half: it reads
//! a definition block into a bounded arena (chapter 20's grammar) and does no
//! I/O of its own.

#![no_std]

extern crate alloc;

pub mod arena;
pub mod block;
pub mod error;
pub mod exec;
pub mod grammar;
pub mod load;
pub mod namespace;
pub mod object;
pub mod op;
pub mod parse;
pub mod region;
pub mod shape;
