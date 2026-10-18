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

//! `OP_GET_APP`. Caller passes a `listing_id`; the kernel client
//! ships it as a length-prefixed string and lifts the response
//! into [`AppSummary`]. The kernel does not consume the per-app
//! detail itself — installer / UI capsules consume it — but the
//! summary keeps the on-the-wire fields in named struct fields so
//! callers do not have to re-implement the byte layout.

mod request;

use alloc::string::String;

pub use request::{get_app, AppSummary};

fn parse(buf: &[u8]) -> Option<AppSummary> {
    let mut cur = buf;
    let listing_id = take_string(&mut cur)?;
    let capsule_id = take_bytes32(&mut cur)?;
    let name = take_string(&mut cur)?;
    let publisher_name = take_string(&mut cur)?;
    let publisher_pubkey = take_bytes32(&mut cur)?;
    let description = take_string(&mut cur)?;
    if cur.len() < 4 {
        return None;
    }
    let release_count = u32::from_le_bytes(cur[0..4].try_into().ok()?);
    Some(AppSummary {
        listing_id,
        capsule_id,
        name,
        publisher_name,
        publisher_pubkey,
        description,
        release_count,
    })
}

fn take_string(cur: &mut &[u8]) -> Option<String> {
    if cur.len() < 4 {
        return None;
    }
    let len = u32::from_le_bytes(cur[0..4].try_into().ok()?) as usize;
    let body = cur.get(4..4 + len)?;
    let s = core::str::from_utf8(body).ok()?.into();
    *cur = &cur[4 + len..];
    Some(s)
}

fn take_bytes32(cur: &mut &[u8]) -> Option<[u8; 32]> {
    if cur.len() < 32 {
        return None;
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&cur[..32]);
    *cur = &cur[32..];
    Some(out)
}
