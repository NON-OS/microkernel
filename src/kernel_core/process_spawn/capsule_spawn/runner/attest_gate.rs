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

use super::super::spec::{CapsuleSpecVerified, SpawnError};
use crate::security::capsule_attest::{verify_capsule_attestation, Proved};

/// Returns what was proved and who proved it, so the spawn path can record
/// both once a pid exists. An empty or refused trailer is an error; this never
/// returns `None`.
pub(crate) fn attest_gate(
    spec: &CapsuleSpecVerified<'_>,
    install_caps: u64,
) -> Result<Option<Proved>, SpawnError> {
    let trailer = spec.attestation_trailer;
    if trailer.is_empty() {
        crate::sys::bench::mark_named(b"capsule_attest_none", spec.name.as_bytes());
        let mut l = crate::sys::serial::Line::new();
        l.str(b"[ZK-ATTEST] none ").str(spec.name.as_bytes()).end();
        return Err(SpawnError::AttestationRejected);
    }
    match verify_capsule_attestation(trailer, spec.elf, install_caps) {
        Ok(proved) => {
            crate::sys::bench::mark_named(b"capsule_attest_ok", spec.name.as_bytes());
            /*
             * The authority is on the line: a reader of the boot log should be
             * able to tell shipped capsules from ones built on this machine.
             */
            let mut l = crate::sys::serial::Line::new();
            l.str(b"[ZK-ATTEST] ok ").str(spec.name.as_bytes());
            l.str(b" ").str(proved.authority.as_str().as_bytes()).end();
            let cert = spec.nonos_id_cert_bytes;
            crate::security::capsule_measure::admit(cert, trailer, proved.is_vendor());
            Ok(Some(proved))
        }
        Err(e) => {
            crate::sys::bench::mark_named(b"capsule_attest_fail", spec.name.as_bytes());
            let mut l = crate::sys::serial::Line::new();
            l.str(b"[ZK-ATTEST] FAIL ").str(spec.name.as_bytes());
            l.str(b": ").str(e.as_str().as_bytes());
            if let crate::security::capsule_attest::AttestError::ProofRefused(code) = e {
                l.str(b" code ").dec(u64::from(code));
            }
            l.end();
            Err(SpawnError::AttestationRejected)
        }
    }
}
