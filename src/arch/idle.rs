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

//! Waiting when there is nothing to run.
//!
//! Two things have to hold however the CPU idles. An interrupt must be able
//! to end the wait, or the wake that ends it can never arrive; and the window
//! between opening them and entering the wait must not be able to swallow a
//! wake that was already pending. Both architectures give exactly one
//! instruction pairing that closes that race, and this is where the kernel
//! asks for it rather than writing either one out.
//!
//! Returns with interrupts closed again, the state the caller was in.

/// Open interrupts, wait for one, close them again.
#[inline]
pub(crate) fn wait_for_interrupt() {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::idle::wait_for_interrupt();
    }
    #[cfg(target_arch = "aarch64")]
    {
        // A PSCI standby state where the tree offers one that pays, else
        // `wfi`. The generic timer counts through both (states that stop it
        // are never entered), so unlike x86_64 no part loses the tick here.
        crate::arch::aarch64::idle_states::wait();
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        core::hint::spin_loop();
    }
}
