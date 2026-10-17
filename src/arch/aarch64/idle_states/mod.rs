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

//! Idle states entered through PSCI CPU_SUSPEND, as the device tree lists
//! them, in place of a plain WFI where one saves more.

mod allow;
mod budget;
mod call;
mod context;
mod describe;
mod enter;
mod flags;
pub mod format;
pub mod layout;
mod powerdown;
mod prepare;
mod psci_format;
mod resume_map;
pub mod rules;
pub mod select;
mod table;

pub use describe::describe;
pub use enter::wait;
pub use prepare::prepare;
