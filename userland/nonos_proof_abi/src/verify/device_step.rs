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

//! Step 2: the device proof is bound to this record and made under roots,
//! a listing and a day the verifier accepts, and it verifies.

use nonos_device_attest::{verify as device_verify, Statement};
use nonos_stark::field::Fp;

use super::pins::{Pins, MAX_DAYS};
use super::refusal::Refusal;
use crate::device::decode_device;
use crate::scope::scope;

fn words(w: &[Fp]) -> [u64; 4] {
    core::array::from_fn(|i| w[i].to_u64())
}

fn trusted(st: &Statement, pins: &Pins) -> bool {
    pins.roots.iter().any(|r| {
        r.boot_root == words(&st.boot_root)
            && r.kernel_root == words(&st.kernel_root)
            && r.device_root == words(&st.device_root)
            && r.device_depth == st.device_depth
    })
}

/// The accepted day whose scope the proof was made under.
fn day_of(st: &Statement, pins: &Pins) -> Option<u64> {
    let (lo, hi) = pins.days;
    if hi < lo || hi - lo >= MAX_DAYS {
        return None;
    }
    let e = [st.scope[0].to_u64(), st.scope[1].to_u64()];
    (lo..=hi).find(|&d| scope(&pins.listing_id, d) == Some(e))
}

/// The tag and the day, once the proof verifies.
pub(super) fn device(b: &[u8], ctx: &[u64; 4], pins: &Pins) -> Result<([u64; 4], u64), Refusal> {
    let (st, proof) = decode_device(b).ok_or(Refusal::DeviceBinding)?;
    if words(&st.context) != *ctx {
        return Err(Refusal::DeviceBinding);
    }
    if !trusted(&st, pins) {
        return Err(Refusal::DeviceTrust);
    }
    let day = day_of(&st, pins).ok_or(Refusal::DeviceTrust)?;
    device_verify(&st, &proof).map_err(|_| Refusal::DeviceProof)?;
    Ok((words(&st.tag), day))
}
