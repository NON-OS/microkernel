// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::embed::{
    DRIVER_RTL8852BE_ATTESTATION_BYTES, DRIVER_RTL8852BE_ELF, DRIVER_RTL8852BE_MANIFEST_BYTES,
    DRIVER_RTL8852BE_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

pub use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

const SERVICE_NAME: &str = "driver.rtl8852be0";
const SERVICE_PORT: u32 = 4268;
const REPLY_INBOX: &str = "endpoint.4294967374";
const REPLY_PORT: u32 = 4269;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub fn spawn_driver_rtl8852be_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: DRIVER_RTL8852BE_ELF,
        nonos_id_cert_bytes: DRIVER_RTL8852BE_NONOS_ID_CERT_BYTES,
        manifest_bytes: DRIVER_RTL8852BE_MANIFEST_BYTES,
        attestation_trailer: DRIVER_RTL8852BE_ATTESTATION_BYTES,
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
        debug_tag: b"[DRIVER-RTL8852BE] load_elf_executable error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
