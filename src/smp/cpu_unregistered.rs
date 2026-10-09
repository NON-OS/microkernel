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

/// A CPU the descriptor table does not know about has no per-CPU block, no
/// current-process slot and no time slice. There is no index it can be given
/// that is not a guess, so it stops here instead of running as another CPU.
pub(super) fn unregistered(apic_id: u32) -> ! {
    crate::sys::serial::print(b"[SMP] FATAL unregistered CPU, APIC id ");
    crate::sys::serial::print_dec(apic_id as u64);
    crate::sys::serial::println(b"");
    crate::arch::halt_loop()
}
