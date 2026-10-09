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

//! What the driver and the reader exchange, as bits: command buffer
//! entries, register and DMA control words, scatter-gather entries, the
//! completion bits, the response bytes and the clock settings. Pure.

pub mod buffer;
pub mod clock_plan;
pub mod encode;
mod kind;
mod outcome;
mod response;

pub use buffer::CmdBuf;
pub use clock_plan::plan;
pub use encode::{haimr_done, haimr_read, haimr_write, hcbctlr, hdbctlr_read, sg_entry};
pub use kind::CmdKind;
pub use outcome::{classify, Outcome};
pub use response::{parse, Response, ResponseError};
