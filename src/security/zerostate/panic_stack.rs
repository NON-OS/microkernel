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

//! The stack the panic wipe keeps, worked out apart from the wipe so the
//! host proofs can place it (kernel_proofs `panic_plan`). The panic path may
//! run on a thread's kernel stack, in the heap or not, or on a CPU's IST
//! stack (a double fault), which lives in the kernel image's .bss. Neither is
//! larger than KSTACK_SIZE, so the window `sp` +/- that size holds all of
//! whichever one `sp` is in, wherever in it `sp` stands.

use super::panic_plan::Span;

/// The kept window around `sp`, clamped at both ends of the address space.
pub(crate) fn stack_keep(sp: u64, kstack: u64) -> Span {
    Span { start: sp.saturating_sub(kstack), end: sp.saturating_add(kstack) }
}
