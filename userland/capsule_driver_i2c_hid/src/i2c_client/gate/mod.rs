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

//! One gate for every call to driver.i2c_pci0.
//!
//! A call the controller driver does not answer costs this driver the whole
//! call timeout and the kernel a `[DIAG] ipc.call unanswered` line. The
//! controller driver cannot answer while it is still bringing its
//! controllers up, and never once it has given up and ended, and this driver
//! asks it something every few milliseconds. So after a few calls in a row
//! go unanswered the gate closes for a pause that doubles each time, from one
//! second to thirty, and every call made while it is closed fails at once
//! without reaching the kernel. The first answered call opens it again and
//! forgets the pauses.

pub mod rule;
mod store;

pub use rule::Gate;
pub use store::{may_call, record};
