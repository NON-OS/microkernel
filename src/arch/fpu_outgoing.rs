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

//! The outgoing task's FP/SIMD registers kept before it can run elsewhere.

/// Keep the current task's FP/SIMD registers in its own slot and leave the
/// unit trapping. Called by a switch before the task is requeued: once it is,
/// another CPU may take it up and load that slot. x86_64 keeps its registers
/// through `process::nonos_core::save_fpu_state` at the same point.
pub(crate) fn save_outgoing_fpu() {
    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::fpu::save_outgoing();
}
