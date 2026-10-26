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

//! The stream bytes of a request, which only the proofs ask for.

use super::ask::{ask, Ask};

/// The stream bytes of a request, for callers that only handle that shape.
pub fn stream_bytes(request: &[u8]) -> Option<&[u8]> {
    match ask(request)? {
        Ask::Stream(bytes) | Ask::Numbered(_, bytes) | Ask::NumberedOn(_, _, bytes) => Some(bytes),
        Ask::Reset | Ask::ResetOn(_) | Ask::Status => None,
    }
}
