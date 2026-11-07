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

//! The hooks as a build without `nonos-bringup-trace` sees them: empty
//! bodies, inlined away at every call site.

use super::ev::Ev;

#[inline(always)]
pub fn event(_dev: &'static str, _ev: Ev) {}

#[inline(always)]
pub fn summary(_stage: &'static str) {}
