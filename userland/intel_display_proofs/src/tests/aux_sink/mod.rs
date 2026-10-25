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

//! A hostile DisplayPort sink on I2C-over-AUX (`sink.rs`) behind DDI B's
//! AUX registers (`regs.rs`), and the EDID read held against it: E-DDC to
//! every block declared, whole blocks only, bounded in tries and time.

pub mod aux_log;
mod bound_tests;
pub mod draw;
mod edge_tests;
pub mod judge;
pub mod regs;
pub mod regs_send;
pub mod sink;
pub mod sink_fault;
pub mod sink_i2c;
pub mod sink_read;
pub mod sink_transfer;
mod sweep_tests;
