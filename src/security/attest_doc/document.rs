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

use alloc::vec::Vec;

#[path = "document_consts.rs"]
mod consts;
pub use consts::*;

/// What the machine hands to whoever asked what it is running.
///
/// `registry_root` and `capsule_count` are carried in the clear for the
/// verifier's convenience, but their authority comes from the quote: the root
/// was folded into the qualifying data the TPM signed. A verifier recomputes
/// that binding rather than trusting these fields.
pub struct AttestationDoc {
    pub challenge: [u8; 32],
    pub registry_root: [u8; 32],
    pub capsule_count: u32,
    /// False once any running capsule could not be recorded. A verifier must
    /// treat a document with this clear as a statement that the machine no
    /// longer knows everything it is running.
    pub registry_complete: bool,
    /// Which IOMMU the machine selected at boot, as one of the `IOMMU_*` values.
    pub iommu_vendor: u8,
    /// Whether that unit translates with this kernel's tables.
    pub iommu_enforcing: bool,
    /// Mappings a device could reach with no IOMMU domain confining them when
    /// the document was produced. With `iommu_enforcing` set and this above
    /// zero, the unit is in service and device DMA still goes around it. Like
    /// the registry root, these are folded into what the TPM signs.
    pub unconfined_grants: u32,
    /// Devices found and not claimed that the unit still lets reach all of
    /// memory. Grants can all be confined while these are not, so DMA on the
    /// machine is confined only when this and `unconfined_grants` are zero.
    pub identity_devices: u32,
    /// The values of `QUOTED_PCRS`, in that order: read just before the
    /// quote, and matched against its digest, so they are the values signed.
    pub pcrs: [[u8; 32]; QUOTED_PCRS.len()],
    /// The firmware's TCG event log as the loader carried it, at most
    /// `MAX_LOG` bytes; empty when it carried none. A verifier replays it to
    /// the quoted values.
    pub log: Vec<u8>,
    /// The `TPMS_ATTEST` the TPM produced, byte for byte as signed.
    pub attest: Vec<u8>,
    pub signature: Vec<u8>,
    /// The attestation key's uncompressed P-256 point, x then y. What a
    /// verifier checks `signature` with. It is derived from the endorsement
    /// seed and a fixed template, so a counterparty that has seen it once can
    /// pin this machine; endorsing it back to the manufacturer is separate.
    pub ak_public: [u8; 64],
}

// The encoder, beside this file so each stays in bounds; a child module, so
// whoever mounts this file by path has the encoder too.
#[path = "document_encode.rs"]
mod encode;
