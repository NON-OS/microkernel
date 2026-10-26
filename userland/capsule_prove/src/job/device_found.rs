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

//! Where the device is enrolled: the registry transcript the person placed,
//! read against the TPM's EK answers and held to a registrar root the
//! release's trust pins for `day`. Needs vfs and the TPM, so it runs on the
//! serving thread (device.rs).

use alloc::vec::Vec;
use core::cell::Cell;

use nonos_proof_abi::trust::release;
use nonos_proof_abi::wire::{NOT_ENROLLED, PROVER, STALE_REGISTRY};

use crate::app::{files, tpm};
use crate::assemble::{enrolled_where, Enrolled, Refusal, TRANSCRIPT_MAX};
use crate::market::{registrar, root_bytes};

use super::status::failed;

const TRANSCRIPT: &[u8] = b"/prove/registry.transcript";

/// The enrolled device and the registrar root it sits under.
pub fn found(day: u64) -> Result<(Enrolled, [u8; 32]), i32> {
    // No pins until the release seals a release: nothing proves before then.
    let trust = release().ok_or(STALE_REGISTRY)?;
    let transcript = files::read(TRANSCRIPT, TRANSCRIPT_MAX).map_err(failed)?;
    let (eks, rc) = tpm::ek_answers();
    if eks.is_empty() {
        return Err(if rc < 0 { failed(rc) } else { NOT_ENROLLED });
    }
    let answers: Vec<&[u8]> = eks.iter().map(|a| a.as_slice()).collect();
    let reg = Cell::new(None);
    let accept = |root, depth| {
        reg.set(registrar(trust.registrars, day, root, depth));
        reg.get().is_some()
    };
    let device = enrolled_where(&transcript, accept, &answers).map_err(refused)?;
    let reg = reg.get().ok_or(STALE_REGISTRY)?;
    Ok((device, root_bytes(&reg)))
}

fn refused(r: Refusal) -> i32 {
    match r {
        Refusal::RootNotRequested => STALE_REGISTRY,
        Refusal::NotEnrolled | Refusal::NoEk => NOT_ENROLLED,
        _ => PROVER,
    }
}
