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

// Hand off to the arch-local primitive. aarch64 and riscv64 will
// land here through the Arch trait once M-ARCH-1 ships.

#[cfg(target_arch = "x86_64")]
pub(super) fn clear_low_half() -> Result<(), &'static str> {
    crate::arch::x86_64::paging::clear_low_half()
}

/*
 * Off x86_64 the kernel still executes from the boot identity map in the low
 * half, so removing it would unmap the code running this. It is kept, and the
 * log says so rather than the caller reporting a teardown that did not happen.
 */
#[cfg(not(target_arch = "x86_64"))]
pub(super) fn clear_low_half() -> Result<(), &'static str> {
    crate::sys::serial::println(
        b"[VM-INIT] low half kept: the kernel still runs from the boot identity map",
    );
    Ok(())
}
