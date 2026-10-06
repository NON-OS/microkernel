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
//! The two fields that say where a body ends.

extern crate alloc;

use alloc::string::String;

use super::super::error::HttpError;

/// What Transfer-Encoding says about the body's framing.
pub(super) enum Codings {
    /// No Transfer-Encoding field.
    None,
    /// The last coding applied is chunked: the chunks frame the body.
    ChunkedLast,
    /// Codings are listed but chunked is not last: the body runs to the close.
    Other,
}

/*
 * The codings of every Transfer-Encoding field, in order, as one list (RFC
 * 9110 5.3). The one applied last frames the body, so "gzip, chunked" is
 * chunked; comparing the whole value with "chunked" read it as not.
 */
pub(super) fn transfer_codings(fields: &[(String, String)]) -> Codings {
    let mut last = None;
    let mut any = false;
    for (_, value) in fields.iter().filter(|(n, _)| n == "transfer-encoding") {
        any = true;
        for coding in value.split(',').map(str::trim).filter(|c| !c.is_empty()) {
            last = Some(coding);
        }
    }
    match (any, last) {
        (false, _) => Codings::None,
        (true, Some(c)) if c.eq_ignore_ascii_case("chunked") => Codings::ChunkedLast,
        (true, _) => Codings::Other,
    }
}

/*
 * Content-Length is digits and nothing else (RFC 9110 8.6). Every value, in
 * every field and every comma separated list, has to state the same length:
 * two lengths are two framings of one stream, the shape of response
 * splitting, and the first one used to win. The integer parser also took a
 * leading sign.
 */
pub(super) fn content_length(fields: &[(String, String)]) -> Result<Option<usize>, HttpError> {
    let mut found: Option<usize> = None;
    for (_, value) in fields.iter().filter(|(n, _)| n == "content-length") {
        for item in value.split(',').map(str::trim) {
            let n = digits(item).ok_or(HttpError::Body)?;
            if found.is_some_and(|f| f != n) {
                return Err(HttpError::Body);
            }
            found = Some(n);
        }
    }
    Ok(found)
}

fn digits(s: &str) -> Option<usize> {
    if s.is_empty() {
        return None;
    }
    s.bytes().try_fold(0usize, |n, b| {
        if !b.is_ascii_digit() {
            return None;
        }
        n.checked_mul(10)?.checked_add(usize::from(b - b'0'))
    })
}
