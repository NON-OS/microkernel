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

//! The interpreter's host: how the capsule reaches what firmware's AML
//! names, each piece through a kernel grant.

mod ec;
mod host_impl;
mod lock;
mod lock_hold;
mod memory_io;
mod pci;
mod ports;
mod region;
mod say;
mod shared;
mod state;
mod tables;
mod windows;

pub use pci::function as pci_function;
pub use shared::{read_shared, write_shared};
pub use state::CapsuleHost;
