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

//! The trace's clock. The cycle counter where it ticks at one rate whatever
//! the core's frequency (CPUID's invariant TSC bit); the uptime milliseconds
//! anywhere else, coarse but never wrong.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    Tsc,
    Millis,
}

impl Source {
    pub fn name(self) -> &'static str {
        match self {
            Source::Tsc => "tsc",
            Source::Millis => "ms",
        }
    }
}

#[cfg(target_arch = "x86_64")]
pub fn source() -> Source {
    use core::arch::x86_64::__cpuid;
    const INVARIANT_TSC: u32 = 1 << 8;
    // CPUID is unprivileged on x86-64.
    let top = __cpuid(0x8000_0000).eax;
    if top < 0x8000_0007 {
        return Source::Millis;
    }
    let pm = __cpuid(0x8000_0007);
    if pm.edx & INVARIANT_TSC != 0 {
        Source::Tsc
    } else {
        Source::Millis
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn source() -> Source {
    Source::Millis
}

pub fn millis() -> u64 {
    u64::try_from(nonos_libc::mk_uptime_ms()).unwrap_or(0)
}

pub fn now(src: Source) -> u64 {
    match src {
        // SAFETY: RDTSC reads a counter; the kernel leaves CR4.TSD clear,
        // so it runs at CPL3.
        #[cfg(target_arch = "x86_64")]
        Source::Tsc => unsafe { core::arch::x86_64::_rdtsc() },
        _ => millis(),
    }
}
