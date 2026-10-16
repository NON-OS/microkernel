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
mod attempt;
mod choose;
mod claim;
mod config;
mod dma;
mod driver;
mod fail;
mod irq;
pub mod irq_plan;
mod linked;
mod mark;
mod mmio;
mod pci;
mod place;
mod play;
mod ring;
mod sequence;
mod take;

pub use driver::{Driver, Started};
pub use fail::mark_setup_fail;
pub(crate) use mark::Line;
pub use sequence::{machine_verdict, run};
