// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::embed::{
    DRIVER_RTL8822CE_ATTESTATION_BYTES, DRIVER_RTL8822CE_ELF, DRIVER_RTL8822CE_MANIFEST_BYTES,
    DRIVER_RTL8822CE_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

pub use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

const SERVICE_NAME: &str = "driver.rtl8822ce0";
const SERVICE_PORT: u32 = 4264;
const REPLY_INBOX: &str = "endpoint.4294967372";
const REPLY_PORT: u32 = 4265;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub fn spawn_driver_rtl8822ce_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: DRIVER_RTL8822CE_ELF,
        nonos_id_cert_bytes: DRIVER_RTL8822CE_NONOS_ID_CERT_BYTES,
        manifest_bytes: DRIVER_RTL8822CE_MANIFEST_BYTES,
        attestation_trailer: DRIVER_RTL8822CE_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::IPC.bit()
            | Capability::Memory.bit()
            // The station address is drawn rather than read out of the efuse, and
            // CryptoRandom is gated on this capability. Without it the draw comes
            // back empty, the PHY is never configured, and the radio stays down.
            | Capability::Crypto.bit()
            | crate::capabilities::serial_debug_cap()
            | Capability::Driver.bit()
            | Capability::DeviceEnum.bit()
            | Capability::Mmio.bit()
            | Capability::Dma.bit(),
        debug_tag: b"[DRIVER-RTL8822CE] load_elf_executable error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
