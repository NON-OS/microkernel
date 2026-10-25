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

//! Whether an embedded capsule is in this kernel, asked before its cert is
//! decoded (capsule_spawn/runner/built_gate.rs, B-02): a verified spawn runs
//! the profile, then the gate, then preflight; a store load refuses an empty
//! image before it spawns.

#[path = "../../../../src/kernel_core/process_spawn/capsule_spawn/runner/built_gate.rs"]
mod rule;

#[cfg(test)]
mod tests;
