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

//! Spawn nsh.front with Network and no AttestRead, as the signed manifest
//! declares (userland/capsule_nsh_front/Capsule.mk). It reaches net.sockets
//! for its listener and nsh.attest for each handshake's document; holding
//! Network, it can never be quoted to directly, which is why the quote comes
//! through nsh.attest.

use super::embed::{
    NSH_FRONT_ATTESTATION_BYTES, NSH_FRONT_ELF, NSH_FRONT_MANIFEST_BYTES,
    NSH_FRONT_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY};

const SERVICE_NAME: &str = "nsh.front";
const SERVICE_PORT: u32 = 4488;
const REPLY_INBOX: &str = "endpoint.nsh.front.reply";
const REPLY_PORT: u32 = 4489;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");
/// CoreExec (bit 0, which MkExit needs), Network, IPC and Memory. No
/// AttestRead: a networked capsule is never handed a quote. Debug is added
/// only by a kernel built with `capsule-serial-debug`, which no image a
/// release loader boots carries (tools/nix/config.nix).
const REQUIRED_CAPS: u64 = 0x0000_001D;

pub fn spawn_nsh_front_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: NSH_FRONT_ELF,
        nonos_id_cert_bytes: NSH_FRONT_NONOS_ID_CERT_BYTES,
        manifest_bytes: NSH_FRONT_MANIFEST_BYTES,
        attestation_trailer: NSH_FRONT_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: REQUIRED_CAPS | crate::capabilities::serial_debug_cap(),
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
