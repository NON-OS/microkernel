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

//! The attack suite's line for each refusal case:
//! `[ATTACK] trailer-<case> refused: <the gate's reason>`, or `ESCAPED` when
//! the spawn gate admitted it. The reason comes from the verifier the gate
//! runs, over the same trailer, ELF and caps.

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
use crate::security::capsule_attest::{verify_capsule_attestation, AttestError};
use crate::sys::serial::{print, print_dec};

pub(super) fn attack_name(case: &[u8]) -> &'static [u8] {
    match case {
        b"flip" => b"trailer-flip",
        b"extra_cap" => b"trailer-extra-cap",
        b"kernel_kind" => b"trailer-kernel-kind",
        b"stale_epoch" => b"trailer-stale-epoch",
        _ => b"trailer-unknown",
    }
}

pub(super) fn attack_line<T>(case: &[u8], spawned: &Result<T, SpawnError>, trailer: &[u8], elf: &[u8], caps: u64) {
    let (refused, how, code) = match spawned {
        Ok(_) => (false, "the spawn gate admitted it", None),
        Err(SpawnError::AttestationRejected) => match verify_capsule_attestation(trailer, elf, caps) {
            Err(e @ AttestError::ProofRefused(c)) => (true, e.as_str(), Some(c)),
            Err(e) => (true, e.as_str(), None),
            Ok(_) => (true, "the spawn gate", None),
        },
        Err(_) => (true, "spawn, before the attestation gate", None),
    };
    print(b"[ATTACK] ");
    print(attack_name(case));
    print(if refused { b" refused: " } else { b" ESCAPED: " });
    print(how.as_bytes());
    if let Some(c) = code {
        print(b" code ");
        print_dec(u64::from(c));
    }
    print(b"\n");
}
