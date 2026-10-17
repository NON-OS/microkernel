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

//! Seeded disks for the kernel's GPT reader: tables built sound at 512-
//! and 4096-byte blocks, one lie told in each, and the rule stated here
//! from the UEFI tables apart from the reader.

pub mod backup_lie;
pub mod build;
pub mod crc;
pub mod disk;
pub mod entry_lie;
pub mod expect;
pub mod head_lie;
pub mod lie;
pub mod lies;
pub mod mbr;
pub mod model;
pub mod parts;
pub mod rng;
pub mod room_lie;
pub mod rule;
pub mod tell;
pub mod view;
