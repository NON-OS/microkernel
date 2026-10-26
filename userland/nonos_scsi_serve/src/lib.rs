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

//! The block wire served for the disks behind a SCSI host adapter.
//!
//! A host driver finds its adapter, brings it up and implements
//! `nonos_scsi::Hba`; this crate does the rest the same way for every
//! adapter: finds the logical units on the adapter's bus (`Units::scan`),
//! says what it found, and serves each unit on the block wire with the
//! commands `nonos_scsi` builds. Units are numbered in the order the scan
//! finds them: by target, then by LUN.

#![no_std]

extern crate alloc;

pub mod say;
pub mod server;
mod units;
// The whole wire as track B's crate has it, some of it for other media.
pub mod wire;

pub use units::{Units, MAX_LUNS, MAX_UNITS};
