// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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
//! Spawn the e1000e driver capsule with its manifest's mask, 0xB8038: PCI MMIO
//! and DMA, polled, so no Irq; frames over IPC, so no Network.

use super::embed::{
    DRIVER_E1000E_ATTESTATION_BYTES, DRIVER_E1000E_ELF, DRIVER_E1000E_MANIFEST_BYTES,
    DRIVER_E1000E_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

pub use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

const SERVICE_NAME: &str = "driver.e1000e_0";
const SERVICE_PORT: u32 = 4270;
const REPLY_PORT: u32 = 4271;
/// The capsule's reply endpoint, CAPSULE_REPLY_ENDPOINT in its Capsule.mk.
const REPLY_INBOX: &str = "endpoint.4294967390";
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub fn spawn_driver_e1000e_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;

    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: DRIVER_E1000E_ELF,
        nonos_id_cert_bytes: DRIVER_E1000E_NONOS_ID_CERT_BYTES,
        manifest_bytes: DRIVER_E1000E_MANIFEST_BYTES,
        attestation_trailer: DRIVER_E1000E_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::IPC.bit()
            | Capability::Memory.bit()
            // The station address is drawn, and CryptoRandom is gated on this.
            | Capability::Crypto.bit()
            // The manifest's optional Debug, for the "e1000e:" lines.
            | crate::capabilities::serial_debug_cap()
            | Capability::Driver.bit()
            | Capability::DeviceEnum.bit()
            | Capability::Mmio.bit()
            | Capability::Dma.bit(),
        debug_tag: b"[DRIVER-E1000E] load_elf_executable error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
