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

//! What the part is and what its bootcode says of the board, the mode words
//! tg3 keeps beside the registers, and the handshake with management
//! firmware when the board runs it.

pub mod fw;
pub mod hw_cfg;
mod identify;
mod modes;
pub mod pcicfg;
pub mod sram;

pub use identify::{identify, Chip};
pub use modes::Modes;
