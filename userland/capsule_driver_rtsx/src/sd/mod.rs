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

//! The SD card protocol the driver speaks once the reader is up, from the
//! SD Physical Layer Simplified Specification: commands, the operating
//! conditions register, the CSD and the card status. Pure.

pub mod bits;
mod command;
mod csd;
pub mod ocr;
mod rsp;
mod status;

pub use command::Command;
pub use csd::capacity_blocks;
pub use ocr::{if_cond_echoed, ocr_high_capacity, ocr_ready};
pub use rsp::Rsp;
pub use status::{r1_app_cmd, r6_rca};
