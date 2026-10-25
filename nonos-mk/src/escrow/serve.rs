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

//! One request through the agent, as HTTP sees it: the minute's bucket first
//! (429), then the answer, and K in the key file before any 200 leaves. A key
//! file that cannot be written stops the agent rather than answer without it.

use std::path::Path;

use super::answer::answer;
use super::answer_types::{Refusal, Served};
use super::bucket::Bucket;
use super::keys::{record, Kept};
use super::seen::Seen;

pub struct Agent<'a> {
    pub served: &'a Served,
    pub seen: Seen,
    pub bucket: Bucket,
    pub keys: &'a Path,
}

impl Agent<'_> {
    pub fn serve(
        &mut self,
        body: &[u8],
        now_ms: u64,
        draw: &mut impl FnMut() -> [u8; 32],
    ) -> std::io::Result<(u16, Vec<u8>)> {
        if !self.bucket.take(now_ms) {
            return Ok((429, Vec::new()));
        }
        let now = now_ms / 1000;
        let a = match answer(self.served, &mut self.seen, body, now, draw) {
            Ok(a) => a,
            Err(Refusal::Seen) => return Ok((429, Vec::new())),
            Err(Refusal::NotServed) => return Ok((404, Vec::new())),
            Err(_) => return Ok((400, Vec::new())),
        };
        let kept = Kept {
            key_commit: &a.key_commit,
            cipher_root: &a.cipher_root,
            buyer_pk: &a.buyer_pk,
            k: &a.k,
        };
        record(self.keys, now, &kept)?;
        Ok((200, a.sealed))
    }
}
