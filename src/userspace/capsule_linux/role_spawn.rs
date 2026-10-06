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

//! The personality spawned in a role: the one verified path every role takes.

use super::embed::{
    LINUX_ATTESTATION_BYTES, LINUX_ELF, LINUX_MANIFEST_BYTES, LINUX_NONOS_ID_CERT_BYTES,
};
use super::roles::Role;
use super::spawn::LINUX_CAPS;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

/// The personality in `role`, parented to the caller, with no argument
/// vector set here: a caller that must hand one over before the process
/// first runs does it from the spawn path (see `terminal::admit`).
pub(super) fn spawn_role(role: &Role) -> Result<u32, SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: role.name,
        service_port: role.port,
        reply_inbox: role.inbox,
        reply_port: role.reply_port,
        elf: LINUX_ELF,
        nonos_id_cert_bytes: LINUX_NONOS_ID_CERT_BYTES,
        manifest_bytes: LINUX_MANIFEST_BYTES,
        attestation_trailer: LINUX_ATTESTATION_BYTES,
        target_triple: env!("NONOS_USER_TARGET"),
        requested_caps: LINUX_CAPS | role.extra_caps,
        debug_tag: role.tag,
    };
    capsule_spawn::spawn_verified(&spec, &trust_anchor, None)
}
