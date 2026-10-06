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

use super::embed::{
    SHIELD_VECTORS_ATTESTATION_BYTES, SHIELD_VECTORS_ELF, SHIELD_VECTORS_MANIFEST_BYTES,
    SHIELD_VECTORS_NONOS_ID_CERT_BYTES,
};
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "shield_vectors";
const SERVICE_PORT: u32 = 4988;
const REPLY_INBOX: &str = "endpoint.shield_vectors.reply";
const REPLY_PORT: u32 = 4989;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub fn spawn_shield_vectors_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;

    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: SHIELD_VECTORS_ELF,
        nonos_id_cert_bytes: SHIELD_VECTORS_NONOS_ID_CERT_BYTES,
        manifest_bytes: SHIELD_VECTORS_MANIFEST_BYTES,
        attestation_trailer: SHIELD_VECTORS_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        // The result lines reach the serial console through Debug.
        requested_caps: Capability::CoreExec.bit()
            | Capability::IPC.bit()
            | Capability::Memory.bit()
            | crate::capabilities::serial_debug_cap(),
        debug_tag: b"",
    };
    capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    Ok(())
}
