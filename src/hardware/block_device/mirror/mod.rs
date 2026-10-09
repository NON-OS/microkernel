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

//! The boot disk's ranges the loader copied into memory: the live plan's
//! sector and the model files it names. Read when no driver of the kernel's
//! drives a disk, so a live boot opens its volume and imports its model
//! whatever stick or controller the machine has.

mod find;
mod record;
mod serve;

pub(super) use serve::{capacity, read};
