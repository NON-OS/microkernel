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

//! A register-level model of the 21140A and 21143, enough of it to hold the
//! driver to the reference manuals and QEMU's tulip: the CSR file eight
//! bytes apart, the software reset, the two processes CSR6 starts and stops,
//! the poll demands, the setup frame loading the perfect filter, and both
//! descriptor rings walked to their end-of-ring marks, every DMA confined
//! to the regions the test granted.

mod csr;
mod events;
mod memory;
mod part;
mod rx;
mod setup_frame;
mod thread;
mod tx;
mod window;

pub use events::{Event, Faults};
pub use memory::{Memory, Region};
pub use part::{Part, RS_STOPPED, RS_SUSPENDED, RS_WAITING};
pub use rx::Delivered;
pub use thread::{install, take, with_part};
