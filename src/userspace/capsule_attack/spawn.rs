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
    ATTACK_ATTESTATION_BYTES, ATTACK_ELF, ATTACK_MANIFEST_BYTES, ATTACK_NONOS_ID_CERT_BYTES,
};
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};
use crate::sys::boot_log;

const SERVICE_NAME: &str = "test.attack";
const SERVICE_PORT: u32 = 4954;
const REPLY_INBOX: &str = "endpoint.test.attack.reply";
const REPLY_PORT: u32 = 4955;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

/// Spawns the attacker through the same gate as every capsule, and says on
/// the serial line whether it ran: a suite that never started fails the
/// checker as "never attempted", and this line says why.
pub fn run() {
    match spawn() {
        Ok(()) => boot_log::ok("SELFTEST", "attack capsule spawned"),
        Err(_) => boot_log::error("SELFTEST attack capsule spawn failed"),
    }
}

fn spawn() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;

    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: ATTACK_ELF,
        nonos_id_cert_bytes: ATTACK_NONOS_ID_CERT_BYTES,
        manifest_bytes: ATTACK_MANIFEST_BYTES,
        attestation_trailer: ATTACK_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        // The least a capsule runs on (Capsule.mk); Debug is required, not
        // optional, because MkDebug is its only way to report.
        requested_caps: Capability::CoreExec.bit()
            | Capability::IPC.bit()
            | Capability::Memory.bit()
            | Capability::Debug.bit(),
        debug_tag: b"",
    };
    capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    Ok(())
}
