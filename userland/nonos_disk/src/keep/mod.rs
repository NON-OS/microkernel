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

//! Updating an installed NØNOS in place, and what an update keeps.
//!
//! An update rewrites the system part alone, the boot partition and the
//! store, and leaves the partition tables, the disk plan, the key header
//! and every sector of the data volume as they are (`session/queue_keep`).
//! That keeps the data only on a disk laid out as this writer lays one out,
//! whose volume the new release can open. [`read_installed`] reads the
//! table, the plan and the key header and gives one of [`Keep`]'s three
//! answers; [`Keep::for_image`] adds whether the new boot files fit.

mod answers;
mod classify;
mod fit;
mod header;
mod key;
mod places;
mod read;
mod source;
mod table;
mod volume;
mod why;

pub(crate) use fit::volume_for;

pub use classify::{Installed, Keep};
pub use read::{names_a_volume, read_installed};
pub use source::DiskAnswers;
pub use why::NotKept;
