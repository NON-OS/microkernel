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

//! FETCH 1 ESCROW (plan/22 step 1, v1.4.24): one POST of the request,
//! without its url, to the release's https endpoint; the sealed answer,
//! agent_pk[32] | ChaCha20-Poly1305 under X25519(agent_pk, buyer_sk), held
//! unopened for the pid that asked, keyed by its buyer_pk, and paged back.
//! Settle never sees inside: it checks only that the answer can hold a key
//! and a tag, and is at most MAX_RELEASE.

use alloc::vec::Vec;

use nonos_nox_config::Chain;
use nonos_settle_proto::{https_target, Escrow, Page, EINVAL, EPROTO, MAX_RELEASE};

use crate::net::post_to;
use crate::settle::{HeldEscrow, Settle};

/// agent_pk and the AEAD's tag: the least a sealed answer can be.
const SEALED_MIN: usize = 32 + 16;

pub fn escrow(s: &mut Settle, sender: u32, e: &Escrow) -> Result<Vec<u8>, i32> {
    Chain::from_id(e.chain_id).ok_or(EINVAL)?;
    let (host, path) = https_target(&e.url).ok_or(EINVAL)?;
    let bytes = post_to(host, path, &e.agent_body(), MAX_RELEASE)?;
    if bytes.len() < SEALED_MIN {
        return Err(EPROTO);
    }
    let first = Page::of(&bytes, 0).ok_or(EPROTO)?.encode();
    s.escrow = Some(HeldEscrow { buyer_pk: e.buyer_pk, pid: sender, bytes });
    Ok(first)
}
