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

//! The outgoing task's user registers kept before it can run elsewhere.

/// Keep the current task's FP/SIMD registers in its own slot, leaving the
/// unit trapping, and on aarch64 its TPIDR_EL0 in its PCB. Called by a
/// switch before the task is requeued: once it is, another CPU may take it
/// up and load both. x86_64 keeps its registers through
/// `process::nonos_core::save_fpu_state` at the same point, and its TLS base
/// changes only through a syscall that records it.
pub(crate) fn keep_outgoing_user_state() {
    #[cfg(target_arch = "aarch64")]
    {
        crate::arch::aarch64::fpu::save_outgoing();
        crate::arch::aarch64::context::keep_outgoing_tls();
    }
}
