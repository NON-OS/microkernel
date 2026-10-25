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

use super::super::artifacts::CapsuleArtifacts;
use super::super::error::LoadError;
use super::endpoint::endpoint;
use crate::kernel_core::process_spawn::capsule_spawn::{
    spawn_verified_as, AttestedParent, CapsuleSpecVerified,
};
use crate::security::capsule_manifest::{decode as decode_manifest, EndpointKind};

// Spawn a capsule whose four artifacts were read from the store by the
// installer. The service name, endpoints, and target triple are taken from the
// capsule's own signed manifest, so a loaded capsule registers exactly what it
// declares and the caller cannot misname or misroute it. The artifacts then go
// through the same verified spawn path baked capsules use, with every identity,
// manifest, publisher signature, capability, and attestation check intact.
// requested_caps may name only what the manifest offers (asked.rs); the
// verified manifest still decides what is actually granted.
pub(crate) fn load_capsule_from_vfs(
    artifacts: CapsuleArtifacts,
    requested_caps: u64,
    args: &[u8],
    on_behalf_of: Option<AttestedParent>,
) -> Result<u32, LoadError> {
    let manifest = decode_manifest(&artifacts.manifest).map_err(|_| LoadError::Manifest)?;
    let (service_name, service_port) = endpoint(&manifest, EndpointKind::Service)?;
    let (reply_name, reply_port) = endpoint(&manifest, EndpointKind::Reply)?;
    super::names::check(service_name, reply_name, manifest.namespace_str())?;
    let requested = super::asked::within_manifest(&manifest, requested_caps, service_name)?;
    super::bound::check(&manifest, requested, on_behalf_of, service_name)?;

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
        name: service_name,
        service_port,
        reply_inbox: reply_name,
        reply_port,
        elf: &artifacts.elf,
        nonos_id_cert_bytes: &artifacts.cert,
        manifest_bytes: &artifacts.manifest,
        attestation_trailer: &artifacts.trailer,
        target_triple: manifest.target_triple_str(),
        requested_caps: requested,
        debug_tag: b"[RUNTIME-LOAD] elf error:",
    };
    let (trust, now_ms) = super::trust::trust_and_now()?;
    let pid = spawn_verified_as(&spec, &trust, now_ms, on_behalf_of).map_err(LoadError::Spawn)?;
    super::args::set_argv(pid, args);
    Ok(pid)
}
