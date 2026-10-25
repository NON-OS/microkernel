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

use alloc::vec::Vec;

use super::http;

#[path = "fetch_body.rs"]
mod fetch_body;
#[path = "fetch_head.rs"]
mod fetch_head;
#[path = "fetch_head_reuse.rs"]
mod fetch_head_reuse;
#[path = "fetch_keep.rs"]
mod fetch_keep;
#[path = "fetch_once.rs"]
mod fetch_once;
#[path = "fetch_retry.rs"]
mod fetch_retry;
#[path = "fetch_reuse.rs"]
mod fetch_reuse;

use fetch_body::decode;
pub use fetch_head_reuse::head_reuse;
pub use fetch_keep::get_on;
pub use fetch_retry::get;
pub use fetch_reuse::get_reuse;

pub const MAX_FETCH: usize = 64 * 1024 * 1024;
const CLOSE_WAIT: u8 = 4;
const EMPTY_BUDGET: u32 = 64;
const RETRIES: u32 = 3;
const BACKOFF_MS: i64 = 250;
const MAX_HOPS: u32 = 5;

pub enum Fetched {
    Body(Vec<u8>),
    Redirect(Vec<u8>),
}

fn is_redirect(status: u16) -> bool {
    matches!(status, 301 | 302 | 303 | 307 | 308)
}

fn finish(raw: &[u8]) -> Result<Fetched, &'static str> {
    let head = http::parse_head(raw).ok_or("bad http response")?;
    if head.status != 200 {
        return match head.location {
            Some(loc) if is_redirect(head.status) => Ok(Fetched::Redirect(loc)),
            _ => Err("http status not 200"),
        };
    }
    if head.chunked {
        return Err("chunked transfer unsupported");
    }
    let body = match head.content_length {
        Some(len) => raw.get(head.body_off..head.body_off + len).ok_or("truncated body")?,
        None => &raw[head.body_off..],
    };
    decode(body, head.gzip).map(Fetched::Body).map_err(|(_, e)| e)
}
