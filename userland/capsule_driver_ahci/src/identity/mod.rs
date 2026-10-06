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

//! What the driver takes from a disk's IDENTIFY DEVICE block. The block is the
//! drive's own bytes, so the decision is made on a copy, without touching
//! hardware, and the host proof crate runs the same source. A block that
//! breaks a rule refuses the disk; nothing is clamped or guessed.

mod address48;
mod capacity;
mod names;
mod refusal;
mod sector;

pub use capacity::capacity;
pub use names::{names, Names};
pub use refusal::Refusal;
