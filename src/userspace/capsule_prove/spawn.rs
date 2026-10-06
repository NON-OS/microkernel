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

//! Opening the prover's window through the verified path every capsule takes,
//! as an on-demand instance: one slot, so one prover at a time, since each
//! holds its heap while it is open.

use super::embed::{
    PROVE_ATTESTATION_BYTES, PROVE_ELF, PROVE_MANIFEST_BYTES, PROVE_NONOS_ID_CERT_BYTES,
};
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{
    spawn_next_instance, InstanceEndpoint, InstanceSpawn, SpawnError,
};

const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

/// The window's endpoints, declared in the signed manifest.
const PROVE_INSTANCES: &[InstanceEndpoint] = &[InstanceEndpoint {
    name: "app.prove.1",
    port: 4952,
    reply_inbox: "endpoint.app.prove.1.reply",
    reply_port: 4953,
}];

/*
 * What its Capsule.mk declares and nothing more: the graphics pair for the
 * window, FileSystem to read the request and the registry, StreamImport to
 * leave the proof on the data volume, Crypto for the blinding entropy and the
 * file's digest, and DeviceSecret, which no other capsule holds. No Network:
 * what it reads was fetched by another capsule, or brought in by the person.
 */
pub const PROVE_CAPS: u64 = Capability::CoreExec.bit()
    | Capability::IPC.bit()
    | Capability::Memory.bit()
    | Capability::Crypto.bit()
    | Capability::FileSystem.bit()
    | Capability::GraphicsDisplayQuery.bit()
    | Capability::GraphicsSurfaceCreate.bit()
    | Capability::StreamImport.bit()
    | Capability::DeviceSecret.bit();

/// The prover's window: a fresh instance when its slot is free, else the
/// running one, to focus. Refused by name in an image built without it.
pub fn spawn_prove_instance() -> Result<u32, SpawnError> {
    if PROVE_ELF.is_empty() {
        return Err(SpawnError::FeatureDisabled);
    }
    spawn_next_instance(&InstanceSpawn {
        elf: PROVE_ELF,
        cert: PROVE_NONOS_ID_CERT_BYTES,
        manifest: PROVE_MANIFEST_BYTES,
        attestation: PROVE_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: PROVE_CAPS | crate::capabilities::serial_debug_cap(),
        instances: PROVE_INSTANCES,
        debug_tag: b"[PROVE-INSTANCE] elf error:",
    })
}
