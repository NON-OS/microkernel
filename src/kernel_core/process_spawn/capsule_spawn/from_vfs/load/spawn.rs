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
use super::spawn_from::{spawn_from, Endpoints};
use crate::kernel_core::process_spawn::capsule_spawn::runner::built_gate::stored;
use crate::kernel_core::process_spawn::capsule_spawn::AttestedParent;
use crate::security::capsule_manifest::{decode as decode_manifest, EndpointKind};

// Spawn a capsule whose four artifacts were read from the store by the
// installer. The service name, endpoints, and target triple are taken from the
// capsule's own signed manifest, so a loaded capsule registers exactly what it
// declares and the caller cannot misname or misroute it. The artifacts then go
// through the same verified spawn path baked capsules use, with every identity,
// manifest, publisher signature, capability, and attestation check intact.
// requested_caps may name only what the manifest offers (asked.rs); the
// verified manifest still decides what is actually granted. An empty image is
// refused here, by name, before a spec exists.
pub(crate) fn load_capsule_from_vfs(
    artifacts: CapsuleArtifacts,
    requested_caps: u64,
    args: &[u8],
    on_behalf_of: Option<AttestedParent>,
) -> Result<u32, LoadError> {
    let manifest = decode_manifest(&artifacts.manifest).map_err(|_| LoadError::Manifest)?;
    let service = endpoint(&manifest, EndpointKind::Service)?;
    let reply = endpoint(&manifest, EndpointKind::Reply)?;
    super::names::check(service.0, reply.0, manifest.namespace_str())?;
    let requested = super::asked::within_manifest(&manifest, requested_caps, service.0)?;
    super::bound::check(&manifest, requested, on_behalf_of, service.0)?;
    let at = Endpoints { service, reply };
    let spawn = || spawn_from(&artifacts, &manifest, at, requested, args, on_behalf_of);
    stored(&artifacts.elf, LoadError::EmptyImage, spawn).inspect_err(|e| {
        if *e == LoadError::EmptyImage {
            crate::sys::serial::print(b"[RUNTIME-LOAD] ");
            crate::sys::serial::print(service.0.as_bytes());
            crate::sys::serial::print(b": its image in the store is empty\n");
        }
    })
}
