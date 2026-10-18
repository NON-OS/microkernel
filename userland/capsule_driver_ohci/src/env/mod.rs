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

//! What the engine needs from where it runs: the controller's registers, a
//! clock for every bounded wait, and the schedule memory the controller
//! reads. The capsule gives it the MMIO grant, the uptime and a DMA region;
//! the host proofs give it a modelled controller.

mod clock;
mod mem;
mod regs;

pub use clock::{hold, until, Clock};
pub use mem::Mem;
pub use regs::Regs;
