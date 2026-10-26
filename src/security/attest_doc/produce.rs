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

use super::binding::qualifying_data;
use super::document::AttestationDoc;
use super::error::AttestDocError;
use super::log::tcg_log;
use super::posture::dma_posture;
use super::produce_quote::quote_pcrs;
use crate::security::attest_registry::{attested_count, registry_complete, registry_root};
use crate::security::capsule_measure::with_events;
use crate::security::tpm::ak::ak_public;
use crate::security::tpm::error::TpmError;

/// Answer "what is this machine running", signed.
///
/// Refuses when the registry is incomplete. A document that omits a running
/// capsule is the one failure a remote party cannot detect, so the machine
/// declines to speak rather than understate itself.
pub(super) fn produce(
    ak_handle: u32,
    challenge: &[u8; 32],
) -> Result<AttestationDoc, AttestDocError> {
    if !registry_complete() {
        return Err(AttestDocError::RegistryIncomplete);
    }
    let root = registry_root();
    let dma = dma_posture();
    let qualifying = qualifying_data(challenge, &root, &dma);
    // The log's copy and the quote under the lock every PCR 14 extend takes,
    // so the log carried is the one whose events the quoted PCR 14 holds.
    let taken = with_events(|events| {
        Ok::<_, AttestDocError>((tcg_log(events)?, quote_pcrs(ak_handle, &qualifying)?))
    });
    let (log, quoted) = taken.ok_or(AttestDocError::MeasureFailed)??;
    // Complete never turns back, so this covers every spawn up to the quote.
    if !registry_complete() {
        return Err(AttestDocError::RegistryIncomplete);
    }

    // Kept when the key was loaded; a handle without it is a driver fault.
    let ak_public = ak_public().ok_or(AttestDocError::Tpm(TpmError::InvalidResponse))?;
    Ok(AttestationDoc {
        challenge: *challenge,
        registry_root: root,
        capsule_count: attested_count() as u32,
        registry_complete: true,
        iommu_vendor: dma.vendor,
        iommu_enforcing: dma.enforcing,
        unconfined_grants: dma.unconfined_grants,
        identity_devices: dma.identity_devices,
        pcrs: quoted.pcrs,
        log,
        attest: quoted.attest,
        signature: quoted.signature,
        ak_public,
    })
}
