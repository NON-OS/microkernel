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

//! The budget one evaluation runs under. Whatever the table asks, an
//! evaluation ends: past any of these it stops with a fault the capsule
//! logs, and the namespace is left as consistent as the last whole step.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecLimits {
    /// Terms evaluated, counted across every call the evaluation makes.
    pub steps: u64,
    /// Method calls nested inside one another.
    pub call_depth: u32,
    /// Bytes of strings, buffers and packages made.
    pub alloc: usize,
    /// Milliseconds spent in Sleep, Stall and timed waits.
    pub time_ms: u64,
}

impl ExecLimits {
    /// Firmware's longest evaluations (an embedded controller poll, a
    /// battery read) take thousands of steps and well under a second; these
    /// leave a hundredfold margin and still end a runaway quickly. The call
    /// depth with the parser's nesting limit bounds the capsule's stack.
    pub const DEFAULT: ExecLimits =
        ExecLimits { steps: 2_000_000, call_depth: 24, alloc: 8 << 20, time_ms: 10_000 };
}

impl Default for ExecLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}
