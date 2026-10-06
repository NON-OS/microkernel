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

//! The settings panel's policy-call error and the pass that reads every stored
//! value when the panel opens, grouped as they sit in the capsule's `ipc`
//! module so the pass's `super::error` resolves the same way. The calls
//! themselves are syscalls and stay out.

#[path = "../../../capsule_settings/src/settings/ipc/error.rs"]
pub mod error;
#[path = "../../../capsule_settings/src/settings/ipc/hydrate_pass.rs"]
pub mod hydrate_pass;
