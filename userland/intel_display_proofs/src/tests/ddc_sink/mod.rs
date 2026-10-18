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

//! A hostile HDMI or DVI sink on the GMBUS path (`sink.rs`), and the EDID
//! and DDC/CI reads held against it: bounded in tries and time, no byte
//! past what was asked, and only whole blocks and checked replies kept.

pub mod bus;
pub mod caps_judge;
pub mod caps_plan;
mod caps_sweep_tests;
pub mod ci_bus;
mod ci_edge_tests;
pub mod ci_gen;
pub mod ci_own;
pub mod draw;
mod eddc_tests;
mod edid_bound_tests;
mod edid_edge_tests;
pub mod edid_gen;
mod edid_sweep_tests;
pub mod judge;
pub mod kit;
pub mod lies;
pub mod own;
pub mod sink;
pub mod sink_command;
pub mod sink_read;
pub mod sink_status;
pub mod sink_write;
pub mod stream;
mod vcp_sweep_tests;
