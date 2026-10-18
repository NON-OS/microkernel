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
    /// Bytes of the capsule's stack the evaluation may use, measured from
    /// where it began: what a call nested inside expressions costs varies
    /// several times over, so the depth alone cannot bound the stack.
    pub stack_bytes: usize,
    /// Bytes of strings, buffers and packages made.
    pub alloc: usize,
    /// Milliseconds spent in Sleep, Stall and timed waits.
    pub time_ms: u64,
}

impl ExecLimits {
    /// Firmware's longest evaluations (an embedded controller poll, a
    /// battery read) take thousands of steps and well under a second; these
    /// leave a hundredfold margin and still end a runaway quickly. Calls
    /// may nest 256 deep, past the 200 some firmware reaches, as long as
    /// they stay within 1 MiB of the capsule's 2 MiB stack: a call costs
    /// about 1.3 KiB nested in a Return and 4.4 KiB four terms down
    /// (measured on the host's release build), and what one method may
    /// still nest after the last check is bounded by the parser's depth.
    pub const DEFAULT: ExecLimits = ExecLimits {
        steps: 2_000_000,
        call_depth: 256,
        stack_bytes: 1 << 20,
        alloc: 8 << 20,
        time_ms: 10_000,
    };
}

impl Default for ExecLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}
