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

//! `TPMS_ATTEST` for a quote and `TPMT_SIGNATURE`, built field by field in
//! the TPM's byte order.

use nsh_attest::quote::{bitmap, pcr_digest};

use super::sig::tpm2b;

#[derive(Clone)]
pub struct Quote {
    pub magic: u32,
    pub st: u16,
    pub signer: Vec<u8>,
    pub extra: Vec<u8>,
    pub safe: u8,
    pub selections: Vec<(u16, Vec<u8>)>,
    pub count: Option<u32>,
    pub digest: Vec<u8>,
    pub trailing: Vec<u8>,
}

impl Quote {
    /// What a TPM answers for a quote over `pcrs`, holding `values`.
    pub fn honest(extra: &[u8], pcrs: &[u8], values: &[[u8; 32]]) -> Self {
        let refs: Vec<&[u8; 32]> = values.iter().collect();
        let mut signer = vec![0x00, 0x0B];
        signer.extend([0x3C; 32]);
        Quote {
            magic: 0xFF54_4347,
            st: 0x8018,
            signer,
            extra: extra.to_vec(),
            safe: 1,
            selections: vec![(0x000B, bitmap(pcrs).to_vec())],
            count: None,
            digest: pcr_digest(&refs).to_vec(),
            trailing: Vec::new(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = self.magic.to_be_bytes().to_vec();
        out.extend(self.st.to_be_bytes());
        tpm2b(&mut out, &self.signer);
        tpm2b(&mut out, &self.extra);
        out.extend(0x0102_0304_0506_0708u64.to_be_bytes());
        out.extend([0, 0, 0, 7, 0, 0, 0, 2, self.safe]);
        out.extend(0x2024_0101u64.to_be_bytes());
        let n = self.count.unwrap_or(self.selections.len() as u32);
        out.extend(n.to_be_bytes());
        for (hash, bits) in &self.selections {
            out.extend(hash.to_be_bytes());
            out.push(bits.len() as u8);
            out.extend(bits);
        }
        tpm2b(&mut out, &self.digest);
        out.extend(&self.trailing);
        out
    }
}
