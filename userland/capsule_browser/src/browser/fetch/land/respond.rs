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

//! A finished fetch's response, and whether it failed for a kept connection.

use alloc::vec::Vec;

use crate::browser::fetch::tls;
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::http;

/// The response bytes of `job`, decrypted when it came over TLS.
pub(in crate::browser::fetch) fn response(job: &mut Fetch) -> Option<Vec<u8>> {
    match job.phase {
        Phase::Decrypt => tls::decrypt(job),
        Phase::Done => Some(core::mem::take(&mut job.buf)),
        _ => None,
    }
}

/// The body of a 200 response, or nothing.
pub(in crate::browser::fetch) fn body_ok(raw: Option<&[u8]>) -> Vec<u8> {
    let resp = raw.and_then(http::response::parse);
    resp.filter(|r| r.status == 200).map(|r| r.body).unwrap_or_default()
}

/// A request sent on a kept connection that drew nothing: the server had
/// closed it. Worth one more try on a fresh connection.
pub(in crate::browser::fetch) fn dead_kept(job: &Fetch) -> bool {
    job.phase == Phase::Error && job.keep_uses > 0 && job.received == 0
}
