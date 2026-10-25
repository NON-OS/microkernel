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

//! nonos.prove.fetch on 5202/5203, after the onion transports and the SOCKS
//! front end it routes through. Caps 0x3D: never DeviceSecret.

use super::embed::{ATTESTATION, CERT, ELF, MANIFEST};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{decode as decode_anchor, BAKED_TRUST_ANCHOR_POLICY};

const SERVICE_NAME: &str = "nonos.prove.fetch";
const SERVICE_PORT: u32 = 5202;
const REPLY_INBOX: &str = "endpoint.nonos.prove.fetch.reply";
const REPLY_PORT: u32 = 5203;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub const FETCH_CAPS: u64 = Capability::CoreExec.bit()
    | Capability::Network.bit()
    | Capability::IPC.bit()
    | Capability::Memory.bit()
    | Capability::Crypto.bit();

pub fn spawn_prove_fetch_capsule() -> Result<(), SpawnError> {
    if ELF.is_empty() {
        return Err(SpawnError::FeatureDisabled);
    }
    let trust = decode_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: ELF,
        nonos_id_cert_bytes: CERT,
        manifest_bytes: MANIFEST,
        attestation_trailer: ATTESTATION,
        target_triple: TARGET_TRIPLE,
        requested_caps: FETCH_CAPS | crate::capabilities::serial_debug_cap(),
        debug_tag: b"[PROVE-FETCH] elf error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust, None)?;
    state::set_alive(pid);
    Ok(())
}
