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

//! Soft-off read ahead, for the panic path.
//!
//! `enter` reads the ACPI tables when it is called. The panic path wipes the
//! heap before it powers off and may take no lock once the wipe has begun, so
//! it reads what the soft-off needs first, here, and enters from that.

/// What soft-off needs, or nothing when the firmware left no way to it.
pub(crate) struct Prepared {
    #[cfg(target_arch = "x86_64")]
    off: Option<crate::arch::x86_64::acpi::power::PreparedOff>,
}

/// Read what soft-off needs without waiting: the ACPI tables' lock is tried,
/// not taken, so a panic raised while it is held gets no soft-off and the
/// machine is wiped and halted instead.
pub(crate) fn prepare() -> Prepared {
    #[cfg(target_arch = "x86_64")]
    use crate::arch::x86_64::acpi::{parser::try_with_data, power::prepare_shutdown};
    Prepared {
        #[cfg(target_arch = "x86_64")]
        off: prepare_shutdown(try_with_data).ok(),
    }
}

/// Power off from `p`: no lock, no allocation, no log. Divergent like `enter`;
/// where the firmware gave no way off, the CPU is parked.
pub(crate) fn enter_prepared(p: Prepared) -> ! {
    #[cfg(target_arch = "x86_64")]
    {
        if let Some(off) = p.off {
            let _ = crate::arch::x86_64::acpi::power::shutdown_prepared(&off);
        }
        crate::arch::halt_loop()
    }
    #[cfg(target_arch = "aarch64")]
    {
        let _ = p;
        crate::arch::aarch64::psci::system_off()
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let _ = p;
        crate::arch::halt_loop()
    }
}
