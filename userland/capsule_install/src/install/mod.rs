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

//! Install NONOS onto a disk, from the image this machine is running.
//!
//! Six screens in a fixed order: what this does, which disk, type its name,
//! writing, reading back, done. Every byte written comes from the memory
//! the bootloader left the image in, so the disk gets exactly what booted
//! and was verified, and the read-back is what says it arrived.

mod app;
mod event;
mod format;
mod job;
mod manifest;
mod source;
mod state;
mod ui;

pub use app::Install;
