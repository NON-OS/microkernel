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

//! The display wire's server side for drivers that scan out of their own
//! memory (PLAN-coverage section 12): a driver implements `Display` for
//! its device and `Platform` for the kernel, and `Server::handle` answers
//! every request the way the wire says.

#![no_std]

pub mod copy;
pub mod display;
pub mod flush;
pub mod handle;
pub mod mode_check;
pub mod mode_set;
pub mod platform;
pub mod primary;
pub mod server;
pub mod slot;
pub mod surface;

pub use copy::copy_rects;
pub use display::{Display, Shadow};
pub use platform::{Platform, Region};
pub use server::Server;
pub use slot::{Slot, MAX_OUTPUTS};
