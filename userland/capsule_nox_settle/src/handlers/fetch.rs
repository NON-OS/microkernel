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

//! FETCH. RELEASE fetches a release document from its https url over the wallet's route, at
//! most MAX_RELEASE, no redirect. It holds the document only if it decodes
//! whole and keccak256 of its v3 signing form, recomputed from the decoded
//! release, is the releaseHash asked; its reply is the first page. PAGE pages
//! on through the one release held, or the escrow answer held for its asker
//! (fetch_escrow.rs); any other key is ENOENT.

use alloc::vec::Vec;

use nonos_settle_proto::{https_target, Fetch, Page, EINVAL, ENOENT, EPROTO, MAX_RELEASE};

use nonos_marketplace_abi::{decode_release_document, release_hash as signed_hash};

use super::fetch_escrow::{escrow, escrow_page};
use crate::keccak::Host;
use crate::net::get;
use crate::settle::{Held, Settle};

pub fn fetch(s: &mut Settle, sender: u32, body: &[u8]) -> Result<Vec<u8>, i32> {
    match Fetch::parse(body).ok_or(EINVAL)? {
        Fetch::Release { url, release_hash } => {
            let (host, path) = https_target(&url).ok_or(EINVAL)?;
            let bytes = get(host, path, MAX_RELEASE)?;
            let release = decode_release_document(&bytes).map_err(|_| EPROTO)?;
            if signed_hash(&Host, &release) != Some(release_hash) {
                return Err(EPROTO);
            }
            let first = Page::of(&bytes, 0).ok_or(EINVAL)?.encode();
            s.held = Some(Held { release_hash, bytes });
            Ok(first)
        }
        Fetch::Page { release_hash: key, offset } => {
            if let Some(page) = escrow_page(s, sender, &key, offset) {
                return page;
            }
            let release = s.held.as_ref().filter(|h| h.release_hash == key).ok_or(ENOENT)?;
            Page::of(&release.bytes, offset).map(|p| p.encode()).ok_or(EINVAL)
        }
        Fetch::Escrow(e) => escrow(s, sender, &e),
    }
}
