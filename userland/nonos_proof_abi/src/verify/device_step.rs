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

//! Step 3: the device proof walks the pinned trees, from a registry good on
//! the proof's day, bound to this record under the scope asked for, and it
//! verifies.

use nonos_device_attest::{verify as device_verify, Statement};
use nonos_stark::field::Fp;

use super::refusal::Refusal;
use crate::device::decode_device;
use crate::trust::Trust;

fn words(w: &[Fp]) -> [u64; 4] {
    core::array::from_fn(|i| w[i].to_u64())
}

fn trusted(st: &Statement, pins: &Trust, window: u64) -> bool {
    let registry = pins.registrars.iter().any(|r| {
        r.root == words(&st.device_root) && r.depth == st.device_depth && r.good_on(window)
    });
    registry && pins.boot_root == words(&st.boot_root) && pins.kernel_root == words(&st.kernel_root)
}

/// The device's tag, once the proof verifies.
pub(super) fn device(
    b: &[u8],
    ctx: &[u64; 4],
    want_scope: [u64; 2],
    window: u64,
    pins: &Trust,
) -> Result<[u64; 4], Refusal> {
    let (st, proof) = decode_device(b).ok_or(Refusal::DeviceBinding)?;
    if words(&st.context) != *ctx {
        return Err(Refusal::DeviceBinding);
    }
    let scope = [st.scope[0].to_u64(), st.scope[1].to_u64()];
    if !trusted(&st, pins, window) || scope != want_scope {
        return Err(Refusal::DeviceTrust);
    }
    device_verify(&st, &proof).map_err(|_| Refusal::DeviceProof)?;
    Ok(words(&st.tag))
}
