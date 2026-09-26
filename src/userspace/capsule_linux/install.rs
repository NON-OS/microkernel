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

//! The personality, spawned to install a package or to run one, rather than
//! to host the built-in program.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::embed::{
    LINUX_ATTESTATION_BYTES, LINUX_ELF, LINUX_MANIFEST_BYTES, LINUX_NONOS_ID_CERT_BYTES,
};
use super::roles::{Role, INSTALL, RUN};
use super::spawn::LINUX_CAPS;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

/// Spawn the installer for `package`, which must hash to `pinned`.
pub fn spawn_install(package: &str, pinned: &[u8; 32]) -> Result<u32, SpawnError> {
    let hex: String = pinned.iter().map(|b| alloc::format!("{b:02x}")).collect();
    spawn(&INSTALL, vec![String::from("install"), String::from(package), hex])
}

/// Spawn the personality to run the program `package` installed.
pub fn spawn_run(package: &str) -> Result<u32, SpawnError> {
    spawn(&RUN, vec![String::from("run"), String::from(package)])
}

fn spawn(role: &Role, argv: Vec<String>) -> Result<u32, SpawnError> {
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
        requested_caps: LINUX_CAPS,
        debug_tag: role.tag,
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    crate::process::with_process(pid, |pcb| *pcb.argv.lock() = argv);
    Ok(pid)
}
