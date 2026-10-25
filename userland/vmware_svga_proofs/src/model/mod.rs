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

//! A model of the SVGA II adapter: the registers behind the port pair, the
//! mode rules QEMU's vmware_vga applies, and the FIFO run on SYNC as QEMU
//! runs it, recording every UPDATE.

mod fifo;
mod regs;
mod svga;
mod thread;

pub use svga::Svga;
pub use thread::{install, take, with_svga};
