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

//! A fixed answer for each kernel read About's screens make.

/// The build's stamp, as `build.rs` would have written it.
pub mod build {
    pub const VERSION: &[u8] = b"0.3.0\n";
    pub const GIT_SHA: &[u8] = b"5097fc6f9d";
    pub const TOOLCHAIN: &[u8] = b"nightly-2026-01-16";
    pub const ARCH: &[u8] = b"x86_64";
}

/// A virtio display at 2560 by 1440.
pub mod display {
    pub fn primary_dimensions() -> Option<(u32, u32)> {
        Some((2560, 1440))
    }

    pub fn virtio_announced() -> bool {
        true
    }
}

/// A clock a day and some hours in, split as the shipped clock splits it.
pub mod uptime {
    pub fn read_millis() -> Option<u64> {
        Some(93_784_567)
    }

    pub fn split_dhms(total_ms: u64) -> (u64, u64, u64, u64) {
        let s = total_ms / 1000;
        (s / 86_400, (s % 86_400) / 3600, (s % 3600) / 60, s % 60)
    }
}

/// The process table's totals.
pub mod runtime {
    pub struct Runtime {
        pub capsules: u32,
        pub mem_total_kb: u64,
        pub mem_used_kb: u64,
        pub load: [u64; 3],
    }

    pub fn sample() -> Option<Runtime> {
        Some(Runtime { capsules: 87, mem_total_kb: 8 << 20, mem_used_kb: 1 << 20, load: [512; 3] })
    }
}

/// The window's own word and admission.
pub mod own {
    use crate::about::data::admission::Admission;

    pub fn held_mask() -> Option<u64> {
        Some(crate::about::data::own_caps::MASK)
    }

    pub fn admission() -> Admission {
        Admission::Proved
    }
}
