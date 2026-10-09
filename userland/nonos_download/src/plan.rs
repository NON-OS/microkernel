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
//! HTTP/1.1 for a client, with no I/O of its own.

//! What one response head means for a download.

use crate::head::Head;
use crate::limit::{MAX_BYTES, TOO_LARGE};
use crate::url::{resolve, DlUrl};

/// Redirects followed before giving up: a CDN takes one or two.
pub const MAX_REDIRECTS: u8 = 5;

#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    /// Ask again at this address.
    Redirect(DlUrl),
    /// The body that follows goes into the file from byte `at` (0 starts the
    /// file over), and the file will be `total` long when known.
    Body { at: u64, total: Option<u64>, chunked: bool },
    /// Stop, and say this.
    Refuse(&'static str),
}

/// Decide from `head`, asked for `url` from byte `from`, after `redirects`.
pub fn decide(head: &Head, url: &DlUrl, from: u64, redirects: u8) -> Decision {
    match head.status {
        301 | 302 | 303 | 307 | 308 => {
            if redirects >= MAX_REDIRECTS {
                return Decision::Refuse("The server redirected the download too many times.");
            }
            let Some(location) = head.location.as_deref() else {
                return Decision::Refuse("The server redirected the download without saying where.");
            };
            match resolve(url, location) {
                Ok(next) => Decision::Redirect(next),
                Err(why) => Decision::Refuse(why),
            }
        }
        200 => {
            // A server that ignores the range sends the whole file again:
            // start the file over rather than append a second copy.
            if head.length.is_some_and(|n| n > MAX_BYTES) {
                return Decision::Refuse(TOO_LARGE);
            }
            Decision::Body { at: 0, total: head.length, chunked: head.chunked }
        }
        206 => match head.range {
            Some((first, total)) if first == from => {
                if total.is_some_and(|n| n > MAX_BYTES) {
                    return Decision::Refuse(TOO_LARGE);
                }
                Decision::Body { at: from, total, chunked: head.chunked }
            }
            _ => Decision::Refuse("The server sent the rest of the file from the wrong place."),
        },
        // Asked from the very end: the file was already whole.
        416 if from > 0 => Decision::Body { at: from, total: Some(from), chunked: false },
        401 | 403 => Decision::Refuse("The server refused the download: it needs a login or does not allow it."),
        404 | 410 => Decision::Refuse("There is no file at that address."),
        429 => Decision::Refuse("The server is limiting downloads; try again later."),
        500..=599 => Decision::Refuse("The server failed to send the file; try again later."),
        _ => Decision::Refuse("The server did not send the file."),
    }
}
