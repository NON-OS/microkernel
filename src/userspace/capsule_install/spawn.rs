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

//! Spawning the installer through the verified path every capsule takes,
//! on demand from the launcher rather than at boot.
//! The capability set is what the installer's Capsule.mk declares and
//! nothing more: Admin is there for the reboot at the end, DeviceEnum for
//! the disk list and the running image, Crypto for the GUIDs it mints,
//! AttestRead for the verdict it shows before a disk is chosen.

use super::embed::{
    INSTALL_ATTESTATION_BYTES, INSTALL_ELF, INSTALL_MANIFEST_BYTES, INSTALL_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "app.install";
const SERVICE_PORT: u32 = 4932;
const REPLY_INBOX: &str = "endpoint.app.install.reply";
const REPLY_PORT: u32 = 4933;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

/// The set both installers run with; the window and the command line do the
/// same work and hold the same authority, declared once.
pub const CLI_CAPS: u64 = Capability::CoreExec.bit()
    | Capability::IPC.bit()
    | Capability::Memory.bit()
    | Capability::Crypto.bit()
    | Capability::Admin.bit()
    | Capability::Debug.bit()
    | Capability::GraphicsDisplayQuery.bit()
    | Capability::GraphicsSurfaceCreate.bit()
    | Capability::DeviceEnum.bit()
    | Capability::AttestRead.bit();

pub fn spawn_install_capsule() -> Result<u32, SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: INSTALL_ELF,
        nonos_id_cert_bytes: INSTALL_NONOS_ID_CERT_BYTES,
        manifest_bytes: INSTALL_MANIFEST_BYTES,
        attestation_trailer: INSTALL_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: CLI_CAPS,
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(pid)
}
