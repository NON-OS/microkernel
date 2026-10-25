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

use super::{AttestationDoc, DOC_MAGIC, DOC_VERSION, QUOTED_PCRS};

impl AttestationDoc {
    /// Length-prefixed and versioned, so a verifier never has to guess where a
    /// field ends. Big-endian throughout to match the TPM structures it
    /// carries, rather than mixing conventions inside one document.
    pub fn encode(&self) -> Vec<u8> {
        let fixed = 128 + 1 + 33 * QUOTED_PCRS.len() + 4;
        let mut out =
            Vec::with_capacity(fixed + self.log.len() + self.attest.len() + self.signature.len());
        out.extend_from_slice(DOC_MAGIC);
        out.extend_from_slice(&DOC_VERSION.to_be_bytes());
        out.extend_from_slice(&self.challenge);
        out.extend_from_slice(&self.registry_root);
        out.extend_from_slice(&self.capsule_count.to_be_bytes());
        out.push(u8::from(self.registry_complete));
        out.push(self.iommu_vendor);
        out.push(u8::from(self.iommu_enforcing));
        out.extend_from_slice(&self.unconfined_grants.to_be_bytes());
        out.extend_from_slice(&self.identity_devices.to_be_bytes());
        out.push(QUOTED_PCRS.len() as u8);
        for (index, value) in QUOTED_PCRS.iter().zip(&self.pcrs) {
            out.push(*index);
            out.extend_from_slice(value);
        }
        out.extend_from_slice(&(self.log.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.log);
        out.extend_from_slice(&(self.attest.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.attest);
        out.extend_from_slice(&(self.signature.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.signature);
        out.extend_from_slice(&(self.ak_public.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.ak_public);
        out
    }
}
