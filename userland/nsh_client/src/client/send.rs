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

//! Sealing what the client sends and opening what it receives, one count
//! per direction. A frame that does not open ends the session unanswered;
//! a spent direction ends it without a frame, since no nonce is left.

use super::Client;
use crate::Refusal;
use alloc::vec;
use alloc::vec::Vec;
use nsh_crypto::CryptoError;
use nsh_wire::{len_field, Expect, Kind, WireError, TAG_LEN};

impl<A, S> Client<A, S> {
    /// Seal one frame for a server waiting at `peer` and queue it.
    pub(super) fn send(&mut self, kind: Kind, body: &[u8], peer: Expect) -> Result<(), Refusal> {
        let len = len_field(body.len(), peer).map_err(Refusal::Unsendable)?;
        let sealer = self.sealer.as_mut().ok_or(Refusal::Ended)?;
        let mut inner = Vec::with_capacity(1 + body.len());
        inner.push(kind.byte());
        inner.extend_from_slice(body);
        let mut sealed = vec![0u8; inner.len() + TAG_LEN];
        let done = sealer.seal(len, &inner, &mut sealed);
        nonos_hash::wipe(&mut inner);
        let n = done.map_err(|e| match e {
            CryptoError::Limit => Refusal::Limit,
            _ => Refusal::Unsendable(WireError::NoRoom),
        })?;
        self.out.extend_from_slice(&len);
        self.out.extend_from_slice(&sealed[..n]);
        Ok(())
    }

    /// Open the bytes after `len`, giving the type byte and body.
    pub(super) fn open(&mut self, len: [u8; 4], sealed: &[u8]) -> Result<Vec<u8>, Refusal> {
        let opener = self.opener.as_mut().ok_or(Refusal::Ended)?;
        let mut inner = vec![0u8; sealed.len().saturating_sub(TAG_LEN)];
        match opener.open(len, sealed, &mut inner) {
            Ok(n) => {
                inner.truncate(n);
                Ok(inner)
            }
            Err(CryptoError::Limit) => Err(Refusal::Limit),
            Err(_) => Err(Refusal::Open),
        }
    }
}
