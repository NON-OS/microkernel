// NONOS Operating System (AGPL-3.0-or-later)
//! Documents built with the kernel's own encoder, and where their fields sit.

use crate::kernel_document::{AttestationDoc, QUOTED_PCRS};

pub(crate) const CHALLENGE: [u8; 32] = [7u8; 32];

/// Where the completeness byte sits, for the tests that write a byte the kernel
/// never would.
pub(crate) const COMPLETE_AT: usize = 8 + 4 + 32 + 32 + 4;
pub(crate) const VENDOR_AT: usize = COMPLETE_AT + 1;
pub(crate) const ENFORCING_AT: usize = VENDOR_AT + 1;
pub(crate) const IDENTITY_AT: usize = ENFORCING_AT + 1 + 4;
/// Where the PCR list starts, and the log after it.
pub(crate) const PCRS_AT: usize = IDENTITY_AT + 4;
pub(crate) const LOG_AT: usize = PCRS_AT + 1 + 33 * QUOTED_PCRS.len();
/// The event log every document here carries.
pub(crate) const LOG: &[u8] = b"a tcg event log";
pub(crate) const ATTEST_LEN_AT: usize = LOG_AT + 4 + LOG.len();

/// Devices left reaching all of memory in every document built here.
pub(crate) const IDENTITY: u32 = 5;

/// A document exactly as the kernel encodes one, from the kernel's own encoder.
pub(crate) fn encoded(
    challenge: &[u8; 32],
    attest: &[u8],
    sig: &[u8],
    count: u32,
    vendor: u8,
    enforcing: bool,
    grants: u32,
) -> Vec<u8> {
    AttestationDoc {
        challenge: *challenge,
        registry_root: [0xAB; 32],
        capsule_count: count,
        registry_complete: true,
        iommu_vendor: vendor,
        iommu_enforcing: enforcing,
        unconfined_grants: grants,
        identity_devices: IDENTITY,
        pcrs: core::array::from_fn(|k| [0x30 + k as u8; 32]),
        log: LOG.to_vec(),
        attest: attest.to_vec(),
        signature: sig.to_vec(),
        ak_public: [0xCD; 64],
    }
    .encode()
}
