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

use alloc::string::String;
use alloc::vec::Vec;

use super::css::css_charset;
use super::guess::{bom, guess};
use super::prescan::prescan;
use super::{encoding, Encoding};

/* What the text is, which decides where its encoding may be declared. */
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Doc {
    Html,
    Css,
    Plain,
}

/* A <meta> is looked for in the first 1024 bytes; a page that is not
UTF-8 and declares nothing there is looked at up to 8 KiB. */
const PRESCAN: usize = 1024;
const PRESCAN_FAR: usize = 8 * 1024;

/* `body` as UTF-8 text. The encoding is the first of: a byte order mark
(removed), the Content-Type charset, a <meta> (HTML) or @charset
(CSS) declaration, then a guess from the bytes. Valid UTF-8 comes
back as it is, without a copy. */
pub fn decode(mut body: Vec<u8>, charset: Option<&[u8]>, doc: Doc) -> Vec<u8> {
    let (enc, skip) = match bom(&body) {
        Some(found) => found,
        None => (choose(&body, charset, doc), 0),
    };
    if enc == Encoding::Utf8 && core::str::from_utf8(&body[skip..]).is_ok() {
        body.drain(..skip);
        return body;
    }
    let mut out = String::with_capacity(body.len() + body.len() / 2);
    enc.decode(&body[skip..], &mut out);
    out.into_bytes()
}

fn choose(b: &[u8], charset: Option<&[u8]>, doc: Doc) -> Encoding {
    if let Some(e) = charset.and_then(encoding) {
        return e;
    }
    let declared = match doc {
        Doc::Html => prescan(b, PRESCAN).or_else(|| {
            core::str::from_utf8(b).is_err().then(|| prescan(b, PRESCAN_FAR)).flatten()
        }),
        Doc::Css => css_charset(b),
        Doc::Plain => None,
    };
    declared.unwrap_or_else(|| guess(b))
}
