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

//! A stand-in `NONOSATT` checker over `"TDOC" | challenge | AK | PCRs`. It
//! can be lax about the challenge, to show the client checks it again,
//! and it counts its calls and remembers whether it ever said yes.

pub use super::doc::{doc, doc_with, DOC_LEN};
use nsh_client::{AttestRefusal, Attester, Quoted};
use std::cell::Cell;

#[derive(Default)]
pub struct TestAttester {
    /// Refuse every document with this.
    pub refuse: Option<AttestRefusal>,
    /// Do not hold the document to the challenge.
    pub lax: bool,
    pub calls: Cell<u32>,
    pub said_ok: Cell<bool>,
    /// Report the challenge at this offset in place of the true one.
    pub challenge_at: Option<usize>,
}

impl Attester for TestAttester {
    fn check(
        &self,
        doc: &[u8],
        challenge: &[u8; 32],
        _: Option<&[u8]>,
    ) -> Result<Quoted, AttestRefusal> {
        self.calls.set(self.calls.get() + 1);
        if doc.len() != DOC_LEN || &doc[..4] != b"TDOC" {
            return Err(AttestRefusal::Parse);
        }
        let got: [u8; 32] = doc[4..36].try_into().unwrap();
        if !self.lax && &got != challenge {
            return Err(AttestRefusal::Challenge);
        }
        if let Some(r) = self.refuse {
            return Err(r);
        }
        let mut firmware_pcrs = [[0u8; 32]; 4];
        for (i, p) in firmware_pcrs.iter_mut().enumerate() {
            p.copy_from_slice(&doc[100 + 32 * i..132 + 32 * i]);
        }
        self.said_ok.set(true);
        Ok(Quoted {
            challenge_at: self.challenge_at.unwrap_or(4),
            ak_public: doc[36..100].try_into().unwrap(),
            firmware_pcrs,
        })
    }
}
