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

//! The prover as a headless job (plan/14): one instance on app.prove.job, no
//! window, serving one PROVE and exiting, from the window's signed ELF and
//! manifest. Job and window never run together: each holds the prover's heap.

use super::embed::{
    PROVE_ATTESTATION_BYTES, PROVE_ELF, PROVE_MANIFEST_BYTES, PROVE_NONOS_ID_CERT_BYTES,
};
use super::job_argv::{job_argv, JobKind};
use super::spawn::{PROVE_CAPS, WINDOW};
use crate::kernel_core::process_spawn::capsule_spawn::{
    spawn_verified, CapsuleSpecVerified, SpawnError,
};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{decode as decode_anchor, BAKED_TRUST_ANCHOR_POLICY};
use crate::services::registry::{lookup_port, lookup_service};

pub const JOB: &str = "app.prove.job";
const JOB_PORT: u32 = 5206;
const JOB_REPLY: &str = "endpoint.app.prove.job.reply";
const JOB_REPLY_PORT: u32 = 5207;

const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");
/// A prover is live, as a job or as the window.
pub fn prover_live() -> bool {
    lookup_service(JOB).is_some()
        || lookup_port(JOB_PORT).is_some()
        || lookup_service(WINDOW).is_some()
}

/// The job for `kind`, bound to `asker`: it serves that pid's frame alone.
pub fn spawn_prove_job(kind: JobKind, asker: u32) -> Result<u32, SpawnError> {
    if PROVE_ELF.is_empty() {
        return Err(SpawnError::FeatureDisabled);
    }
    if prover_live() {
        return Err(SpawnError::EndpointCollision);
    }
    let trust = decode_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: JOB,
        service_port: JOB_PORT,
        reply_inbox: JOB_REPLY,
        reply_port: JOB_REPLY_PORT,
        elf: PROVE_ELF,
        nonos_id_cert_bytes: PROVE_NONOS_ID_CERT_BYTES,
        manifest_bytes: PROVE_MANIFEST_BYTES,
        attestation_trailer: PROVE_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: PROVE_CAPS | crate::capabilities::serial_debug_cap(),
        debug_tag: b"[PROVE-JOB] elf error:",
    };
    let pid = spawn_verified(&spec, &trust, None)?;
    crate::process::with_process(pid, |pcb| {
        *pcb.argv.lock() = job_argv(kind, asker);
    });
    Ok(pid)
}
