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

//! How long one MkIrqWait sleeps (plan/54 K6, A10 gap 1). Pure, and proven
//! on the host (kernel_proofs::irq_wait_bound). Every wait has a deadline:
//! 0 asks for the default, and anything longer than MAX_WAIT_MS is cut to
//! it. A driver that wants longer waits again; the call returns 1 on a
//! timeout, so it can tell the cut from an interrupt.

pub(crate) const DEFAULT_WAIT_MS: u64 = 100;
pub(crate) const MAX_WAIT_MS: u64 = 5_000;

pub(crate) fn wait_ms(timeout_ms: u64) -> u64 {
    match timeout_ms {
        0 => DEFAULT_WAIT_MS,
        t => t.min(MAX_WAIT_MS),
    }
}
