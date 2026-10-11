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

//! The engine's surroundings in a NONOS capsule: device discovery and claim
//! through the broker, the register window, DMA regions, the uptime clock
//! and the serial console. The only part of the tree that uses `nonos_libc`
//! and `alloc`.

mod discover;
mod dma;
mod env;
mod open;

pub use discover::{discover, Found};
pub use open::{open, Opened};
