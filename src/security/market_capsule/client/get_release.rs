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

//! `OP_GET_RELEASE`. Returns the on-the-wire release record. The
//! kernel surfaces the raw bytes plus the four leading fields a
//! caller commonly wants (release_id, manifest_hash, package_hash,
//! package_url); deeper parsing belongs in the installer capsule
//! that will consume the package.

mod request;

use alloc::string::String;
use alloc::vec::Vec;

pub(crate) use request::queued_get_release;
pub use request::{get_release, ReleaseSummary};

fn parse(buf: Vec<u8>) -> Option<ReleaseSummary> {
    let mut cur = buf.as_slice();
    let release_id = take_string(&mut cur)?;
    let manifest_hash = take_bytes32(&mut cur)?;
    let package_hash = take_bytes32(&mut cur)?;
    let package_url = take_string(&mut cur)?;
    Some(ReleaseSummary { release_id, manifest_hash, package_hash, package_url, raw: buf })
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
