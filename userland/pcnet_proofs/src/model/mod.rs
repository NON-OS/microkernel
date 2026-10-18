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

//! A register-level model of the Am79C970A, enough of it to hold the driver
//! to the data sheet: the indexed registers behind RAP, the software reset,
//! BCR20's software style, INIT reading the init block, STRT, TDMD, and both
//! descriptor rings, every DMA confined to the regions the test granted.

mod csr0;
mod events;
mod memory;
mod part;
mod rx;
mod thread;
mod tx;
mod window;
mod wire;

pub use events::{Event, Faults, Rings};
pub use memory::{Memory, Region};
pub use part::Part;
pub use thread::{install, take, with_part};
pub use wire::Delivered;
