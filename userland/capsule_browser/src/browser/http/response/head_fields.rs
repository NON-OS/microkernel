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

use super::bytes::list;
use super::codings::{Coding, Codings};
use super::content_length::{content_length, Length};
use super::fields::each_field;
use super::head::{Framing, Head, Scan};
use super::location::location;
use super::mime::Mime;

/* The fields of the final response, and its framing: rule 1 of RFC 9112
6.3 (no content for 1xx, 204, 304), then Transfer-Encoding over
Content-Length (chunked only when it is the final coding), then an
invalid Content-Length as an error, then the length, then close. */
pub fn read_fields(fields: &[u8], status: u16, http10: bool, body_at: usize) -> Scan {
    let (mut length, mut content, mut transfer) = (Length::Absent, Codings::NONE, Codings::NONE);
    let (mut mime, mut loc, mut close, mut keep) = (Mime::NONE, None, false, false);
    each_field(fields, |name, value| {
        let is = |s: &[u8]| name.eq_ignore_ascii_case(s);
        if is(b"content-length") {
            content_length(&mut length, value);
        } else if is(b"transfer-encoding") {
            transfer.push_list(value);
        } else if is(b"content-encoding") {
            content.push_list(value);
        } else if is(b"content-type") {
            mime.feed(value);
        } else if is(b"location") {
            loc = Some(location(value));
        } else if is(b"connection") {
            for token in list(value) {
                close |= token.eq_ignore_ascii_case(b"close");
                keep |= token.eq_ignore_ascii_case(b"keep-alive");
            }
        }
    });
    let chunked_last = transfer.as_slice().last() == Some(&Coding::Chunked);
    let framing = if (100..200).contains(&status) || status == 204 || status == 304 {
        Framing::Empty
    } else if transfer.present {
        if chunked_last {
            Framing::Chunked
        } else {
            Framing::Close
        }
    } else {
        match length {
            Length::Absent => Framing::Close,
            Length::Is(n) => Framing::Length(n),
            Length::Invalid => return Scan::Bad,
        }
    };
    close |= http10 && !keep;
    Scan::Head(Head { status, body_at, framing, content, transfer, mime, location: loc, close })
}
