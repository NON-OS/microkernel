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

//! The kernel's reserved and runtime-registrable names (mounted as shipped),
//! and the registry and register right they are asked against (`held`).

mod held;
#[path = "../../../../../src/services/registry/reserved.rs"]
mod reserved;

pub(crate) use held::{caller_has_register_right, hold, lookup_service, set_right};
pub(crate) use reserved::{is_reserved_service, is_runtime_registrable};
