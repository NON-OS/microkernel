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

//! Deciding, once and on the boot CPU, whether return addresses are signed.

use super::secret::{boot_secret, Source};
use super::slot::bind;
use super::state::{active, kernel_key, publish, record};
use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::cpu::id::mpidr_affinity;
use crate::arch::aarch64::cpu::{has_feature, CpuFeature};
use crate::arch::aarch64::security::bti::built_with_pac_ret;
use crate::crypto::hash::blake3_derive_key;

const KERNEL_CONTEXT: &str = "NONOS 2026-10 pointer authentication kernel IA key v1";
const USER_CONTEXT: &str = "NONOS 2026-10 pointer authentication user keys v1";

pub(super) const ABSENT: u8 = 3;
pub(super) const NOT_BUILT: u8 = 4;
pub(super) const NO_ENTROPY: u8 = 5;

/// The kernel key for `install_kernel_key!`, or `None`, having recorded why,
/// where the core lacks FEAT_PAuth, the image lacks pac-ret, or nothing at
/// boot can key it. Before any secondary starts, so all agree.
pub fn prepare() -> Option<[u64; 2]> {
    if !has_feature(CpuFeature::Pauth) {
        record(ABSENT);
        return None;
    }
    if !built_with_pac_ret() {
        record(NOT_BUILT);
        return None;
    }
    let Some((secret, source)) = boot_secret() else {
        record(NO_ENTROPY);
        return None;
    };
    let (mut kernel, mut user) = ([0u8; 16], [0u8; 32]);
    blake3_derive_key(KERNEL_CONTEXT, &secret, &mut kernel);
    blake3_derive_key(USER_CONTEXT, &secret, &mut user);
    let word = |at: usize| u64::from_le_bytes(kernel[at..at + 8].try_into().unwrap_or([0; 8]));
    let key = [word(0), word(8)];
    publish(key, &user, source as u8);
    Some(key)
}

/// A secondary's key, if the boot CPU turned signing on. A secondary that
/// cannot sign cannot run a thread whose frames another CPU signed.
pub fn secondary_key() -> Result<Option<[u64; 2]>, &'static str> {
    if !active() {
        return Ok(None);
    }
    if !has_feature(CpuFeature::Pauth) {
        return Err("the boot CPU signs return addresses and this CPU cannot");
    }
    Ok(Some(kernel_key()))
}

/// Point this CPU's TPIDR_EL1 at its key slot, once the roster can name it.
/// Changes no key, so any function may call it.
pub fn bind_this_cpu() -> bool {
    if !active() {
        return false;
    }
    roster::index_of(mpidr_affinity()).is_some_and(|cpu| bind(cpu, kernel_key()))
}

impl Source {
    pub(super) const fn from_status(status: u8) -> Option<Self> {
        match status {
            1 => Some(Source::Rndr),
            2 => Some(Source::DeviceTreeSeed),
            _ => None,
        }
    }
}
