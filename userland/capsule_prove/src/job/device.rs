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
//! plan/15): the registry transcript the person placed, held to a registrar
//! root trust pins; the boot slots; the device secret, wiped as soon as the
//! statement holds it.

use alloc::vec::Vec;
use core::cell::Cell;

use nonos_proof_abi::encode_device;
use nonos_proof_abi::trust::release;
use nonos_proof_abi::wire::{NOT_ENROLLED, PROVER, STALE_REGISTRY};
use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use crate::app::{files, tpm};
use crate::assemble::{bound, enrolled_where, slots, wipe, wipe_bytes, Refusal, TRANSCRIPT_MAX};
use crate::market::{registrar, root_bytes};

use super::status::failed;

const TRANSCRIPT: &[u8] = b"/prove/registry.transcript";

pub fn proof(day: u64, (e, context): ([Fp; 2], [Fp; RATE])) -> Result<Vec<u8>, i32> {
    // No pins until the owner seals a release: nothing proves before then.
    let trust = release().ok_or(STALE_REGISTRY)?;
    let transcript = files::read(TRANSCRIPT, TRANSCRIPT_MAX).map_err(failed)?;
    let (eks, rc) = tpm::ek_answers();
    if eks.is_empty() {
        return Err(if rc < 0 { failed(rc) } else { NOT_ENROLLED });
    }
    let answers: Vec<&[u8]> = eks.iter().map(|a| a.as_slice()).collect();
    let found = Cell::new(None);
    let accept = |root, depth| {
        found.set(registrar(trust.registrars, day, root, depth));
        found.get().is_some()
    };
    let device = enrolled_where(&transcript, accept, &answers).map_err(refused)?;
    let reg = found.get().ok_or(STALE_REGISTRY)?;
    let record = tpm::slots_record().map_err(failed)?;
    let slots = slots(&record).map_err(|_| PROVER)?;
    let mut secret = [0u8; 32];
    let rc = tpm::secret(&mut secret);
    let made = (rc == 0).then(|| bound(e, context, &root_bytes(&reg), device, slots, &secret));
    wipe_bytes(&mut secret);
    let (st, mut w) = made.ok_or(failed(rc))?.map_err(|_| PROVER)?;
    let mut entropy = [0u8; 64];
    let drawn = nonos_libc::crypto_random(entropy.as_mut_ptr(), entropy.len());
    let proof = (drawn >= 0).then(|| nonos_device_attest::prove(&st, &w, &entropy));
    wipe(&mut w);
    wipe_bytes(&mut entropy);
    encode_device(&st, &proof.ok_or(PROVER)?.map_err(|_| PROVER)?).ok_or(PROVER)
}

fn refused(r: Refusal) -> i32 {
    match r {
        Refusal::RootNotRequested => STALE_REGISTRY,
        Refusal::NotEnrolled | Refusal::NoEk => NOT_ENROLLED,
        _ => PROVER,
    }
}
