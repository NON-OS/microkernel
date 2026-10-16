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

//! Hot-plug and docking, the AML half (ACPI 6.5, 5.6.6 and 6.3): which
//! Notify values ask for a check, which devices a check covers, and the
//! methods that eject and dock. What a device's leaving costs its driver
//! is the kernel's to enforce; this only reads and runs firmware's side.

mod check;
mod dock;
mod eject;
mod targets;

pub use check::{check, Check};
pub use dock::{dependents, dock, is_dock};
pub use eject::{eject, ejectable, EjectError};
pub use targets::{present, targets};
