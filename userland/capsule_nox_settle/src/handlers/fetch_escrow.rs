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

//! FETCH 1 ESCROW (plan/22 step 1, plan/29): one POST of the request, less
//! its url, to the release's https endpoint; the sealed answer held unopened,
//! one at a time, for the pid that asked, keyed by its buyer_pk, paged back.
//! Settle checks only that it can hold a key and a tag, at most MAX_RELEASE.

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;
use nonos_nox_config::Chain;
use nonos_settle_proto::{https_target, Escrow, Page, EINVAL, ENOENT, EPROTO, MAX_RELEASE};

use crate::net::post_to;
use crate::settle::{HeldEscrow, Settle};

/// agent_pk and the AEAD's tag: the least a sealed answer can be.
const SEALED_MIN: usize = 32 + 16;
/// A held escrow answer nobody pages for this long is dropped.
const ESCROW_IDLE_MS: u64 = 300_000;

pub fn escrow(s: &mut Settle, sender: u32, e: &Escrow) -> Result<Vec<u8>, i32> {
    Chain::from_id(e.chain_id).ok_or(EINVAL)?;
    let (host, path) = https_target(&e.url).ok_or(EINVAL)?;
    let bytes = post_to(host, path, &e.agent_body().ok_or(EINVAL)?, MAX_RELEASE)?;
    if bytes.len() < SEALED_MIN {
        return Err(EPROTO);
    }
    let first = Page::of(&bytes, 0).ok_or(EPROTO)?.encode();
    s.escrow = Some(HeldEscrow {
        buyer_pk: e.buyer_pk,
        pid: sender,
        bytes,
        at_ms: mk_time_millis().max(0) as u64,
    });
    Ok(first)
}

/// A PAGE of the held escrow answer, for its asker and key alone: None when
/// it is not theirs. Past ESCROW_IDLE_MS without a page it is gone (ENOENT);
/// once its last page is served it goes too (v1.4.29).
pub fn escrow_page(
    s: &mut Settle,
    sender: u32,
    key: &[u8; 32],
    offset: u32,
) -> Option<Result<Vec<u8>, i32>> {
    let e = s.escrow.as_mut().filter(|e| e.buyer_pk == *key && e.pid == sender)?;
    let now = mk_time_millis().max(0) as u64;
    if now.saturating_sub(e.at_ms) > ESCROW_IDLE_MS {
        s.escrow = None;
        return Some(Err(ENOENT));
    }
    let Some(page) = Page::of(&e.bytes, offset) else { return Some(Err(EINVAL)) };
    e.at_ms = now;
    if offset as usize + page.bytes.len() == e.bytes.len() {
        s.escrow = None;
    }
    Some(Ok(page.encode()))
}
