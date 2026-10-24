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

use super::secret::boot_secret;
use super::state::{publish, record};
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
