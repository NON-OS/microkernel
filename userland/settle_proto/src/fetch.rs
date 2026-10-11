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

//! OP_FETCH. The first byte is the form:
//! - 0 RELEASE: lp(u16) https url, at most MAX_URL | releaseHash[32];
//! - 2 PAGE: releaseHash[32] | offset u32, the rest of the one release settle
//!   holds;
//! - 1 ESCROW: the plan step 1, B5; EINVAL until then.
//!
//! The reply to RELEASE and PAGE is a Page.

use alloc::vec::Vec;

pub const FETCH_RELEASE: u8 = 0;
pub const FETCH_ESCROW: u8 = 1;
pub const FETCH_PAGE: u8 = 2;
pub const MAX_URL: usize = 256;
/// The most one reply carries.
pub const PAGE_LEN: usize = 60 * 1024;
/// The largest release settle fetches.
pub const MAX_RELEASE: usize = 1024 * 1024;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Fetch {
    Release { url: Vec<u8>, release_hash: [u8; 32] },
    Page { release_hash: [u8; 32], offset: u32 },
    Escrow,
}

impl Fetch {
    /// None for ESCROW, whose body is B5's, or a url past MAX_URL.
    pub fn encode(&self) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        match self {
            Fetch::Release { url, release_hash } if url.len() <= MAX_URL => {
                out.push(FETCH_RELEASE);
                out.extend_from_slice(&(url.len() as u16).to_le_bytes());
                out.extend_from_slice(url);
                out.extend_from_slice(release_hash);
            }
            Fetch::Page { release_hash, offset } => {
                out.push(FETCH_PAGE);
                out.extend_from_slice(release_hash);
                out.extend_from_slice(&offset.to_le_bytes());
            }
            _ => return None,
        }
        Some(out)
    }
}
