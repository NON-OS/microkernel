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

//! The raw client's frames: sealing whatever a proof asks for, and opening
//! what the server sends.

use super::raw::Raw;
use nsh_wire::{Kind, TAG_LEN};

impl Raw {
    /// Seal a type byte and body, whatever they are, with a length field
    /// that matches them.
    pub fn seal_inner(&mut self, inner: &[u8]) -> Vec<u8> {
        let len = u32::try_from(inner.len() + TAG_LEN).expect("len").to_le_bytes();
        let mut out = vec![0u8; inner.len() + TAG_LEN];
        self.sealer.as_mut().expect("keys").seal(len, inner, &mut out).expect("seal");
        [&len[..], &out].concat()
    }

    pub fn seal(&mut self, kind: Kind, body: &[u8]) -> Vec<u8> {
        self.seal_inner(&[&[kind.byte()][..], body].concat())
    }

    /// Open every frame in `wire`, as (type, body); stops at the first
    /// that does not open.
    pub fn open(&mut self, mut wire: &[u8]) -> Vec<(u8, Vec<u8>)> {
        let mut frames = Vec::new();
        while wire.len() >= 4 {
            let len: [u8; 4] = wire[..4].try_into().expect("len");
            let n = u32::from_le_bytes(len) as usize;
            let Some(sealed) = wire.get(4..4 + n) else { break };
            let mut inner = vec![0u8; n.saturating_sub(TAG_LEN)];
            let Some(o) = self.opener.as_mut() else { break };
            if o.open(len, sealed, &mut inner).is_err() {
                break;
            }
            frames.push((inner[0], inner[1..].to_vec()));
            wire = &wire[4 + n..];
        }
        frames
    }
}
