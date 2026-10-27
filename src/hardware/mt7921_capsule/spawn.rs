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
    DRIVER_MT7921_ATTESTATION_BYTES, DRIVER_MT7921_ELF, DRIVER_MT7921_MANIFEST_BYTES,
    DRIVER_MT7921_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

pub use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

const SERVICE_NAME: &str = "driver.mt7921e0";
const SERVICE_PORT: u32 = 4264;
const REPLY_INBOX: &str = "endpoint.4294967372";
const REPLY_PORT: u32 = 4265;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub fn spawn_driver_mt7921_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: DRIVER_MT7921_ELF,
        nonos_id_cert_bytes: DRIVER_MT7921_NONOS_ID_CERT_BYTES,
        manifest_bytes: DRIVER_MT7921_MANIFEST_BYTES,
        attestation_trailer: DRIVER_MT7921_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::IPC.bit()
            | Capability::Memory.bit()
            // The station address is drawn rather than the card's own, and
            // CryptoRandom is gated on this capability. Without it the draw comes
            // back empty and the radio stays down.
            | Capability::Crypto.bit()
            | crate::capabilities::serial_debug_cap()
            | Capability::Driver.bit()
            // Also what MkInstallSource is gated on: the firmware package.
            | Capability::DeviceEnum.bit()
            | Capability::Mmio.bit()
            | Capability::Dma.bit(),
        debug_tag: b"[DRIVER-MT7921] load_elf_executable error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
