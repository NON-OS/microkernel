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

//! The verified spawn of a store-loaded capsule, once its image is known to
//! be there: the spec its manifest names, through spawn_verified_as.

use super::super::artifacts::CapsuleArtifacts;
use super::super::error::LoadError;
use crate::kernel_core::process_spawn::capsule_spawn::{
    spawn_verified_as, AttestedParent, CapsuleSpecVerified,
};
use crate::security::capsule_manifest::CapsuleManifest;

/// The service and reply endpoints the manifest names.
#[derive(Clone, Copy)]
pub(super) struct Endpoints<'a> {
    pub(super) service: (&'a str, u32),
    pub(super) reply: (&'a str, u32),
}

pub(super) fn spawn_from(
    artifacts: &CapsuleArtifacts,
    manifest: &CapsuleManifest,
    at: Endpoints<'_>,
    requested_caps: u64,
    args: &[u8],
    on_behalf_of: Option<AttestedParent>,
) -> Result<u32, LoadError> {
    /*
     * Borrowed from artifacts, which this function owns and drops on the way
     * out, on every path. It used to leak all four blobs into the kernel heap
     * before the signature was checked, so a caller holding nothing more than
     * the CoreExec, IPC and Memory every application has could spend 64 MB of
     * ring-0 memory per call and never give it back: four calls exhausted the
     * 256 MB heap and the allocator's failure path halts the machine.
     *
     * The same leak ran on the success path, where it cost a daily driver its
     * memory one app launch at a time, since nothing in process teardown knew
     * the allocations existed.
     */
    let spec = CapsuleSpecVerified {
        name: at.service.0,
        service_port: at.service.1,
        reply_inbox: at.reply.0,
        reply_port: at.reply.1,
        elf: &artifacts.elf,
        nonos_id_cert_bytes: &artifacts.cert,
        manifest_bytes: &artifacts.manifest,
        attestation_trailer: &artifacts.trailer,
        target_triple: manifest.target_triple_str(),
        requested_caps,
        debug_tag: b"[RUNTIME-LOAD] elf error:",
    };
    let (trust, now_ms) = super::trust::trust_and_now()?;
    let pid = spawn_verified_as(&spec, &trust, now_ms, on_behalf_of).map_err(LoadError::Spawn)?;
    super::args::set_argv(pid, args);
    Ok(pid)
}
