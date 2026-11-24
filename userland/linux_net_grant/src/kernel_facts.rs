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

//! The facts a front asks of the kernel itself: MkCapCheck for Network, and
//! MkFamilyFact for the family, which answers only the processes registered
//! as net.anon and net.socks5. A refusal or an error is read as no.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_cap_check, mk_family_argv, mk_family_generation, mk_family_holds};

use super::{Facts, LOCAL, PUBLISHER};

const _: () = assert!(LOCAL == nonos_libc::FAMILY_KIND_LOCAL);
const _: () = assert!(PUBLISHER == nonos_libc::FAMILY_KIND_PUBLISHER);

/// Capability::Network, bit 2 of the kernel's capability mask.
const NETWORK: u64 = 1 << 2;
/// More than any argv a Terminal's `linux` run is given.
const ARGV_MAX: usize = 4096;

pub struct KernelFacts;

impl Facts for KernelFacts {
    fn holds_network(&self, pid: u32) -> bool {
        mk_cap_check(pid, NETWORK)
    }

    fn holds(&self, pid: u32, measurement: &[u8; 32], kinds: u8) -> bool {
        mk_family_holds(pid, measurement, kinds) == 1
    }

    fn generation(&self, pid: u32) -> u64 {
        u64::try_from(mk_family_generation(pid)).unwrap_or(0)
    }

    fn argv(&self, pid: u32) -> Option<Vec<u8>> {
        let mut out = vec![0u8; ARGV_MAX];
        let n = usize::try_from(mk_family_argv(pid, &mut out)).ok()?;
        (n <= ARGV_MAX).then(|| {
            out.truncate(n);
            out
        })
    }
}
