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

//! The record's chain, footer and signature, and its checkpoints.

use super::keys::{self, Hybrid};
use super::record::{Rec, SESSION};
use nonos_hash::{sha512, Sha512};
use nsh_crypto::{labelled, Label};

fn link(parts: &[&[u8]]) -> [u8; 64] {
    let mut h = Sha512::new();
    parts.iter().for_each(|p| h.update(p));
    h.finalize()
}

impl Rec {
    /// `c(seq + 1)`: the chain after entry `seq`.
    pub fn chain_after(&self, seq: usize) -> [u8; 64] {
        let start = link(&[b"nsh/1 record\0", &self.header]);
        self.entries[..=seq].iter().fold(start, |c, e| link(&[&c, e]))
    }

    /// Everything the footer's signature covers.
    pub fn unsigned(&self) -> Vec<u8> {
        let mut out = self.header.clone();
        self.entries.iter().for_each(|e| out.extend_from_slice(e));
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.end_ms.to_le_bytes());
        out.push(1);
        out.extend_from_slice(&self.chain_after(self.entries.len() - 1));
        out
    }

    pub fn signed_by(&self, key: &Hybrid) -> Vec<u8> {
        let body = self.unsigned();
        let sig = key.sign(&labelled(Label::RecordSig, &sha512(&body)));
        [body, sig].concat()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.signed_by(keys::host())
    }

    pub fn ckpt_with(&self, session: [u8; 16], seq: u32, chain: [u8; 64], key: &Hybrid) -> Vec<u8> {
        let mut out = b"NSHC".to_vec();
        out.extend_from_slice(&[1, 0, 0, 0]);
        out.extend_from_slice(&session);
        out.extend_from_slice(&seq.to_le_bytes());
        out.extend_from_slice(&chain);
        let sig = key.sign(&labelled(Label::RecordCkpt, &sha512(&out)));
        [out, sig].concat()
    }

    pub fn ckpt(&self, seq: u32) -> Vec<u8> {
        self.ckpt_with(SESSION, seq, self.chain_after(seq as usize), keys::host())
    }
}
