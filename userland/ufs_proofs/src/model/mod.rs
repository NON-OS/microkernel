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

//! A UFSHCI host with a UFS device behind it, at register level, served
//! synchronously: a UIC command or a doorbell completes inside the write
//! that starts it.

mod cfg;
mod dev;
mod doorbell;
mod lu;
mod mem;
mod mmio;
mod rig;
mod rsp;
mod scsi;
mod scsi_inquiry;
mod scsi_io;
mod scsi_report;
mod scsi_unit;
mod sim;
mod state;
mod uic;

pub use cfg::Cfg;
pub use lu::Lu;
pub use rig::Rig;
