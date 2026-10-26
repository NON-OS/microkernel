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

//! The device proof under a scope and context the caller derived (plan/14,
//! plan/15), with the device secret wiped as soon as the statement holds
//! it. What needs a service or the TPM is gathered on the serving thread
//! (`inputs`, device_found.rs); the proof itself runs on the worker
//! (`prove`, worker.rs), so a STATUS meanwhile is answered at once.

use alloc::vec::Vec;

use nonos_device_attest::{Statement, Witness};
use nonos_proof_abi::encode_device;
use nonos_proof_abi::wire::PROVER;
use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use crate::app::tpm;
use crate::assemble::{bound, slots, wipe, wipe_bytes};

use super::status::failed;

/// What the worker proves: the statement, its witness and fresh entropy,
/// both wiped when it drops, proven or not.
pub struct Device {
    st: Statement,
    w: Witness,
    entropy: [u8; 64],
}

impl Drop for Device {
    fn drop(&mut self) {
        wipe(&mut self.w);
        wipe_bytes(&mut self.entropy);
    }
}

pub fn inputs(day: u64, (e, context): ([Fp; 2], [Fp; RATE])) -> Result<Device, i32> {
    let (device, root) = super::device_found::found(day)?;
    let record = tpm::slots_record().map_err(failed)?;
    let slots = slots(&record).map_err(|_| PROVER)?;
    let mut secret = [0u8; 32];
    let rc = tpm::secret(&mut secret);
    let made = (rc == 0).then(|| bound(e, context, &root, device, slots, &secret));
    wipe_bytes(&mut secret);
    let (st, w) = made.ok_or(failed(rc))?.map_err(|_| PROVER)?;
    let mut d = Device { st, w, entropy: [0; 64] };
    d.entropy = super::entropy::drawn().ok_or(PROVER)?;
    Ok(d)
}

/// Worker: the proof, encoded.
pub fn prove(d: Device) -> Result<Vec<u8>, i32> {
    let proof = nonos_device_attest::prove(&d.st, &d.w, &d.entropy).map_err(|_| PROVER)?;
    encode_device(&d.st, &proof).ok_or(PROVER)
}
