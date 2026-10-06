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

/// Wire format version. A verifier that does not recognise it must refuse
/// rather than parse optimistically: a document it half understands is worse
/// than one it rejects.
pub const DOC_VERSION: u32 = 3;

pub const DOC_MAGIC: &[u8; 8] = b"NONOSATT";

/// `iommu_vendor` values. A verifier that meets another value must refuse.
pub const IOMMU_NONE: u8 = 0;
pub const IOMMU_INTEL_VTD: u8 = 1;
pub const IOMMU_AMD_VI: u8 = 2;

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
    /// zero, the unit is in service and device DMA still goes around it; only
    /// the two together say whether DMA on the machine is confined. Like the
    /// registry root, all three are folded into what the TPM signs.
    pub unconfined_grants: u32,
    /// The `TPMS_ATTEST` the TPM produced, byte for byte as signed.
    pub attest: Vec<u8>,
    pub signature: Vec<u8>,
    /// The attestation key's uncompressed P-256 point, x then y. What a
    /// verifier checks `signature` with. It is derived from the endorsement
    /// seed and a fixed template, so a counterparty that has seen it once can
    /// pin this machine; endorsing it back to the manufacturer is separate.
    pub ak_public: [u8; 64],
}

impl AttestationDoc {
    /// Length-prefixed and versioned, so a verifier never has to guess where a
    /// field ends. Big-endian throughout to match the TPM structures it
    /// carries, rather than mixing conventions inside one document.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(128 + self.attest.len() + self.signature.len());
        out.extend_from_slice(DOC_MAGIC);
        out.extend_from_slice(&DOC_VERSION.to_be_bytes());
        out.extend_from_slice(&self.challenge);
        out.extend_from_slice(&self.registry_root);
        out.extend_from_slice(&self.capsule_count.to_be_bytes());
        out.push(u8::from(self.registry_complete));
        out.push(self.iommu_vendor);
        out.push(u8::from(self.iommu_enforcing));
        out.extend_from_slice(&self.unconfined_grants.to_be_bytes());
        out.extend_from_slice(&(self.attest.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.attest);
        out.extend_from_slice(&(self.signature.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.signature);
        out.extend_from_slice(&(self.ak_public.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.ak_public);
        out
    }
}
