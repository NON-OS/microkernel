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

//! What the engine needs from where it runs. The capsule gives it the mapped
//! register window, the uptime clock and the serial console (`platform`);
//! the host proofs give it a model controller, a counting clock and a log
//! they can read back.

mod clock;
mod dma_buf;
mod log;
mod mmio;

pub use clock::{pause, Clock, Deadline};
pub use dma_buf::DmaBuf;
pub use log::Log;
pub use mmio::Mmio;
