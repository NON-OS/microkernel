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

//! FETCH. RELEASE fetches a release's signing bytes from its https url over
//! the wallet's route, at most MAX_RELEASE, no redirect, and holds them only
//! if keccak256 of them is the releaseHash asked; its reply is the first
//! page. PAGE pages on through the one release held; any other hash is
//! ENOENT. ESCROW is B5's, EINVAL until then.

use alloc::vec::Vec;

use nonos_settle_proto::{https_target, Fetch, Page, EINVAL, ENOENT, EPROTO, MAX_RELEASE};

use crate::keccak::keccak;
use crate::net::get;
use crate::settle::{Held, Settle};

pub fn fetch(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    match Fetch::parse(body).ok_or(EINVAL)? {
        Fetch::Release { url, release_hash } => {
            let (host, path) = https_target(&url).ok_or(EINVAL)?;
            let bytes = get(host, path, MAX_RELEASE)?;
            if keccak(&bytes) != release_hash {
                return Err(EPROTO);
            }
            let first = Page::of(&bytes, 0).ok_or(EINVAL)?.encode();
            s.held = Some(Held { release_hash, bytes });
            Ok(first)
        }
        Fetch::Page { release_hash, offset } => {
            let held = s.held.as_ref().filter(|h| h.release_hash == release_hash).ok_or(ENOENT)?;
            Page::of(&held.bytes, offset).map(|p| p.encode()).ok_or(EINVAL)
        }
        Fetch::Escrow => Err(EINVAL),
    }
}
