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

//! The transcript chain (nsh-v1-wire.md, "The transcript"). Each frame
//! adds one link, `H(previous || n | type | body)` with `n = 1 + body
//! length`, the plaintext length whether or not the frame was sealed. A
//! signed frame's prefix is a link over the body without its signatures,
//! made without changing the chain.

use nonos_hash::{sha512, Sha512};
use nsh_wire::Kind;

const START: &[u8] = b"nsh/1 transcript\0";

pub struct Transcript {
    th: [u8; 64],
}

impl Default for Transcript {
    fn default() -> Self {
        Transcript { th: sha512(START) }
    }
}

fn link(th: &[u8; 64], kind: Kind, body: &[u8]) -> [u8; 64] {
    // Every body nsh/1 allows is under the handshake cap, far below u32.
    let n = u32::try_from(1 + body.len()).unwrap_or(u32::MAX);
    let mut h = Sha512::new();
    h.update(th);
    h.update(&n.to_le_bytes());
    h.update(&[kind.byte()]);
    h.update(body);
    h.finalize()
}

impl Transcript {
    /// Add a frame, sent or received, in the order it crossed the wire.
    pub fn push(&mut self, kind: Kind, body: &[u8]) {
        self.th = link(&self.th, kind, body);
    }

    /// `TH(F without its signatures)` for a frame not yet pushed, given the
    /// bytes its signatures cover (`signed` from the frame's parser).
    pub fn without_sigs(&self, kind: Kind, signed: &[u8]) -> [u8; 64] {
        link(&self.th, kind, signed)
    }

    /// `TH` of the last frame pushed.
    pub fn digest(&self) -> [u8; 64] {
        self.th
    }
}
