//! What PCR 4 may hold. The TPM extends only an event's digest, never its
//! type, so a type is only a claim: an application relabelled as an action
//! would replay the same. PCR 4 therefore admits four types alone, and a
//! separator or action must carry the SHA-256 of its own body, so the
//! only digests in PCR 4 the client cannot recompute are applications,
//! and every one of those is held to the policy.

use nonos_hash::sha256;

use super::{Event, EV_EFI_BOOT_SERVICES_APPLICATION, EV_NO_ACTION};
use crate::LogFault;

pub const EV_SEPARATOR: u32 = 0x0000_0004;
pub const EV_EFI_ACTION: u32 = 0x8000_0007;

pub(super) fn pcr4_event(e: &Event<'_>) -> Result<(), LogFault> {
    match e.kind {
        EV_NO_ACTION | EV_EFI_BOOT_SERVICES_APPLICATION => Ok(()),
        EV_SEPARATOR | EV_EFI_ACTION if sha256(e.body) == e.sha256 => Ok(()),
        EV_SEPARATOR | EV_EFI_ACTION => Err(LogFault::Pcr4Digest),
        _ => Err(LogFault::Pcr4Event),
    }
}
