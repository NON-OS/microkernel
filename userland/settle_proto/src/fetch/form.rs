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
//! - 2 PAGE: key[32] | offset u32, the rest of what settle holds under that
//!   key: the one release, by releaseHash, or the one escrow answer, by
//!   buyer_pk and to the pid that asked for it alone;
//! - 1 ESCROW: plan/22 step 1 (fetch_escrow.rs).
//!
//! The reply to each is a Page.

use alloc::vec::Vec;

use super::escrow::Escrow;

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
    Release {
        url: Vec<u8>,
        release_hash: [u8; 32],
    },
    /// `release_hash` is the key: a release's hash, or a buyer_pk.
    Page {
        release_hash: [u8; 32],
        offset: u32,
    },
    Escrow(Escrow),
}

impl Fetch {
    /// None for a url or solution past its limit.
    pub fn encode(&self) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        match self {
            Fetch::Release { url, release_hash } if url.len() <= MAX_URL => {
                out.push(FETCH_RELEASE);
                out.extend_from_slice(&crate::bytes::len_u16(url.len()).to_le_bytes());
                out.extend_from_slice(url);
                out.extend_from_slice(release_hash);
            }
            Fetch::Page { release_hash, offset } => {
                out.push(FETCH_PAGE);
                out.extend_from_slice(release_hash);
                out.extend_from_slice(&offset.to_le_bytes());
            }
            Fetch::Escrow(e) => return e.encode(),
            _ => return None,
        }
        Some(out)
    }
}
