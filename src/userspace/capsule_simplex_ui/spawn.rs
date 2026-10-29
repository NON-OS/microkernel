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

//! Spawning the chat window through the verified path every capsule takes,
//! on demand from the launcher rather than at boot. The capability set is
//! what userland/capsule_simplex_ui/Capsule.mk declares and nothing more: no
//! Network and no FileSystem, since the window reaches the simplex-chat core
//! only through its SERVE_ONE channel and keeps nothing on disk.

use super::embed::{
    SIMPLEX_UI_ATTESTATION_BYTES, SIMPLEX_UI_ELF, SIMPLEX_UI_MANIFEST_BYTES,
    SIMPLEX_UI_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "app.simplex";
const SERVICE_PORT: u32 = 5014;
const REPLY_INBOX: &str = "endpoint.app.simplex.reply";
const REPLY_PORT: u32 = 5015;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub(super) fn spawn_simplex_ui_capsule() -> Result<u32, SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: SIMPLEX_UI_ELF,
        nonos_id_cert_bytes: SIMPLEX_UI_NONOS_ID_CERT_BYTES,
        manifest_bytes: SIMPLEX_UI_MANIFEST_BYTES,
        attestation_trailer: SIMPLEX_UI_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::CoreExec.bit()
            | Capability::IPC.bit()
            | Capability::Memory.bit()
            | Capability::GraphicsDisplayQuery.bit()
            | Capability::GraphicsSurfaceCreate.bit()
            | crate::capabilities::serial_debug_cap(),
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(pid)
}
