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

//! What a closed connection means for the fetch that was reading it.
//!
//! A proxy says when the far end finished, and that used to be thrown away:
//! a fetch whose exit had hung up waited out its whole silent budget (three
//! minutes over Nym) and then said "timed out", and a page sent with no
//! length was only believed whole after a long quiet. A close now ends the
//! fetch in the call that sees it.

use crate::browser::http::response::{ends_at_close, is_complete};

/// The far end closed before this request heard a byte. Not tried again:
/// the exit (or the host behind it) hung up on purpose, and a second try
/// meets the same answer after the same wait.
pub const EXIT_CLOSED: &str = "the exit closed the connection";

/// What a response amounts to when its connection closed after `raw`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AtClose {
    /// Not a byte of it arrived.
    Nothing,
    /// It is whole: its framing is satisfied, or it had none and ran until
    /// the close, as RFC 9112 6.3 lets a response do.
    Whole,
    /// It stopped short of what its head declared, or of a head at all.
    Truncated,
}

pub fn at_close(raw: &[u8]) -> AtClose {
    if raw.is_empty() {
        AtClose::Nothing
    } else if is_complete(raw) || ends_at_close(raw) {
        AtClose::Whole
    } else {
        AtClose::Truncated
    }
}
