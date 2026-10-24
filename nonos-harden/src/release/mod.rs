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

//! The checks over the sealed release itself: its record, the bytes it
//! names, and what kind of image it is.

mod config;
mod entry;
pub mod kernel;
mod kernel_bind;
pub mod kernel_check;
mod kernel_key_files;
pub mod kernel_keys;
pub mod kind;
pub mod load;
pub mod path;
pub mod read;
pub mod receipt;
mod receipt_agree;
pub mod receipt_check;
mod receipt_parts;
pub mod record;
pub mod resolve;
pub mod sealed;
mod sealed_dir;
pub mod walk;

/// The record the seal writes beside what it sealed.
pub const RECORD: &str = "nonos-release.json";
