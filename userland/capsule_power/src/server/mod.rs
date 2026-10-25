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

/// The sender check, as capsule_acpi makes it: whether a pid's process owns
/// a service now, from the registry and the kernel's thread groups.
#[path = "../../../capsule_acpi/src/server/caller.rs"]
mod caller;
mod handlers;
mod respond;
mod runner;

pub use handlers::prepare_soft_off;
pub use runner::run;
