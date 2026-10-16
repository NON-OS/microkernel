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

//! The shell's pure trusted-path rules, under the `state` path they name
//! each other by.

#[path = "../../../capsule_desktop_shell/src/state/attention.rs"]
pub mod attention;
#[path = "../../../capsule_desktop_shell/src/state/input_source_rule.rs"]
pub mod input_source_rule;
#[path = "../../../capsule_desktop_shell/src/state/trusted_rule.rs"]
pub mod trusted_rule;
