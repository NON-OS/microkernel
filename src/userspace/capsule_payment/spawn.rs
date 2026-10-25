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
    PAYMENT_ATTESTATION_BYTES, PAYMENT_ELF, PAYMENT_MANIFEST_BYTES, PAYMENT_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "payment";
const SERVICE_PORT: u32 = 4114;
const REPLY_INBOX: &str = "endpoint.4294967323";
const REPLY_PORT: u32 = 4115;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");
// CoreExec | IPC | Memory | Crypto | FileSystem | Keyring; kept hand-synced with
// userland/capsule_payment/Capsule.mk's CAPSULE_REQUIRED_CAPS and the manifest
// signed from it.
const REQUIRED_CAPS: u64 = 0x1000_0079;

pub fn spawn_payment_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: PAYMENT_ELF,
        nonos_id_cert_bytes: PAYMENT_NONOS_ID_CERT_BYTES,
        manifest_bytes: PAYMENT_MANIFEST_BYTES,
        attestation_trailer: PAYMENT_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: REQUIRED_CAPS,
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
