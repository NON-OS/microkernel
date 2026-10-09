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

//! This boot's slots, then the secret, asked for last: after the request, the
//! registry and the slots have all checked out. The secret's buffer is wiped
//! inside the step that read it, whatever the step's answer.

use alloc::string::String;

use super::state::{Outcome, Step, Work};
use super::{errno, tpm};
use crate::assemble::{assemble, slots, wipe_bytes};

pub fn boot_slots(w: &mut Work) -> Outcome {
    let record =
        tpm::slots_record().map_err(|e| alloc::format!("Boot slots: {}", errno::slots(e)))?;
    let s = slots(&record).map_err(|r| alloc::format!("Boot slots refused: {}", r.why()))?;
    w.slots = Some(s);
    Ok((
        "Boot slots: the bootloader's and the kernel's, from the kernel".into(),
        Some(Step::Secret),
    ))
}

pub fn secret(w: &mut Work) -> Outcome {
    let missing = || String::from("A step before this one did not finish");
    let req = w.request.as_ref().ok_or_else(missing)?;
    let device = w.enrolled.take().ok_or_else(missing)?;
    let slots = w.slots.take().ok_or_else(missing)?;
    let mut secret = [0u8; 32];
    let rc = tpm::secret(&mut secret);
    if rc != 0 {
        wipe_bytes(&mut secret);
        return Err(alloc::format!("Device secret: {}", errno::secret(rc)));
    }
    let assembled = assemble(req, device, slots, &secret);
    wipe_bytes(&mut secret);
    let (statement, witness) =
        assembled.map_err(|r| alloc::format!("Device secret refused: {}", r.why()))?;
    w.statement = Some(statement);
    w.witness = Some(witness);
    Ok(("Device secret: it is the one the registry enrolled".into(), Some(Step::Primed)))
}
