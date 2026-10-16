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

//! Bringing up the command (CORB) and response (RIRB) rings.
//!
//! Two things here follow Linux rather than a literal reading of the
//! specification. The CORB read pointer reset is a handshake (write RST, see it
//! read 1, write 0, see it read 0), but some controllers clear the bit on
//! their own and never show the 1, and Linux carries on after logging the
//! timeout (`azx_clear_corbrp`, sound/hda/hdac_controller.c); refusing the
//! controller there threw away a working ring. And the ring size is the
//! largest the controller says it supports in CORBSIZE/RIRBSIZE bits 7:4,
//! rather than 256 written blind: a controller offering only 16 entries
//! wrapped at 16 while the driver counted to 256.

mod read_pointer;
mod rings;
mod size;

pub use rings::*;
pub use size::*;
