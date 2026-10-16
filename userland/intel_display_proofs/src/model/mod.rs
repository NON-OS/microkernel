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

pub mod aux_hw;
pub mod bus;
pub mod cable;
pub mod clock;
pub mod dc_hw;
pub mod ddi_hw;
pub mod expiry;
pub mod firmware;
pub mod frames;
pub mod gen9;
pub mod gmbus_ddc_hw;
pub mod gmbus_ddc_read_hw;
pub mod gmbus_hw;
pub mod gmbus_state;
pub mod irq_hw;
pub mod latch;
pub mod mccs;
pub mod mccs_aux;
pub mod mccs_reply;
pub mod mmio;
pub mod pipe_hw;
pub mod regs;
pub mod scanout;
pub mod shared;
pub mod sink;
pub mod sink_aux;
pub mod sink_both;
pub mod sink_link;
pub mod sink_status;
pub mod south;

pub use clock::Ticks;
pub use expiry::Polls;
pub use frames::ModelFrames;
pub use gen9::Gen9;
pub use shared::Shared;
