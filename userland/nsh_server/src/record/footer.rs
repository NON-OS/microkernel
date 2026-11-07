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

//! The footer: how many entries, when and why the session ended, the
//! chain's final value, then the host keys' signature over
//! `H("nsh/1 record-sig\0" || SHA-512(every byte before the signature))`.

use super::Record;
use crate::EndReason;
use nonos_hash::sha512;
use nsh_crypto::{labelled, Label};
use nsh_wire::record::FOOTER_LEN;
use nsh_wire::HybridSig;

impl Record {
    /// Write the footer's fields and give the digest to sign. Room for
    /// them was kept back by every entry.
    pub(crate) fn close(&mut self, now_ms: u64, why: EndReason) -> [u8; 64] {
        let end_ms = now_ms.max(self.last_ms);
        self.bytes.reserve_exact(FOOTER_LEN);
        self.bytes.extend_from_slice(&self.count.to_le_bytes());
        self.bytes.extend_from_slice(&end_ms.to_le_bytes());
        self.bytes.push(why.code());
        let chain = self.chain;
        self.bytes.extend_from_slice(&chain);
        labelled(Label::RecordSig, &sha512(&self.bytes))
    }

    /// The signature that ends the record.
    pub(crate) fn seal(&mut self, sig: &HybridSig<'_>) {
        self.bytes.extend_from_slice(sig.ed);
        self.bytes.extend_from_slice(sig.ml);
    }
}
