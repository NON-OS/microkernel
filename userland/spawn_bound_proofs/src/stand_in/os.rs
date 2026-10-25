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

//! The kernel modules the bound calls into, over the stand-in's state.

pub mod process {
    pub mod core {
        pub(crate) use crate::ambient::AMBIENT_CAPS;
    }
    pub fn current_pid() -> Option<u32> {
        super::super::CALLER.get()
    }
    pub mod caps {
        pub fn bits(pid: u32) -> Option<u64> {
            super::super::super::CAPS.with(|c| c.borrow().get(&pid).copied())
        }
        pub fn has(pid: u32, mask: u64) -> bool {
            bits(pid).map(|b| b & mask == mask).unwrap_or(false)
        }
    }
}

pub mod syscall {
    pub mod microkernel {
        pub mod errnos {
            pub const ERRNO_PERM: i64 = -1;
        }
    }
}

pub mod sys {
    pub mod serial {
        pub fn println(b: &[u8]) {
            let t = String::from_utf8_lossy(b);
            super::super::super::SERIAL.with(|s| s.borrow_mut().push_str(&format!("{t}\n")));
        }
    }
}

pub mod security {
    pub mod capsule_manifest {
        pub struct CapsuleManifest {
            pub required_caps: u64,
            pub optional_caps: u64,
        }
        /// A manifest too short to hold the two capability words.
        #[derive(Debug)]
        pub struct Short;
        /// Stands in for the decoder: required then optional, big-endian.
        pub fn decode(m: &[u8]) -> Result<CapsuleManifest, Short> {
            let word = |at: usize| m.get(at..at + 8).map(|b| u64::from_be_bytes(b.try_into().unwrap()));
            Ok(CapsuleManifest { required_caps: word(0).ok_or(Short)?, optional_caps: word(8).ok_or(Short)? })
        }
    }
}
