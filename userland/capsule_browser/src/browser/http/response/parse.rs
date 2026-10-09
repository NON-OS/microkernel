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

use super::charset;
use super::decode_body::decode_body;
use super::head::{scan, Scan};
use super::kind::{binary, doc, is_script};
use super::mime::Mime;
use super::types::{ContentKind, Response};

/* The final response of `raw`. A page or other text whose body stops
short (a connection cut before Content-Length or the last chunk, a
gzip stream without its trailer, a page past the inflate cap) comes
back with the part that arrived, so it renders; images, fonts and
scripts come back only whole. Text is transcoded to UTF-8 from the
encoding its bytes, header or <meta> declare. */
pub fn parse(raw: &[u8]) -> Option<Response> {
    let Scan::Head(h) = scan(raw) else { return None };
    let kind = content_kind(&h.mime);
    let text = matches!(kind, ContentKind::Html | ContentKind::Text);
    let (body, complete) = decode_body(&raw[h.body_at..], &h, text && !is_script(&h.mime))?;
    /* Without a Content-Type the bytes decide: binary ones are not text. */
    let text = text && (h.mime.present || !binary(&body));
    if !(complete || text && !is_script(&h.mime)) {
        return None;
    }
    let body = if text {
        charset::decode(body.into_owned(), h.mime.charset.as_deref(), doc(kind, &h.mime))
    } else {
        body.into_owned()
    };
    Some(Response { status: h.status, body, location: h.location, content_kind: kind })
}

/* HTML for text/html and XHTML, text for any other text/ type and JSON,
and the kind this browser does not render for everything else. A
response without Content-Type is taken as HTML, as it always was. */
fn content_kind(m: &Mime) -> ContentKind {
    if !m.present {
        return ContentKind::Html;
    }
    let e = &m.essence[..];
    let has = |s: &[u8]| e.windows(s.len()).any(|w| w == s);
    if has(b"text/html") || has(b"application/xhtml") {
        ContentKind::Html
    } else if e.starts_with(b"text/") || has(b"application/json") {
        ContentKind::Text
    } else {
        ContentKind::Unsupported
    }
}
