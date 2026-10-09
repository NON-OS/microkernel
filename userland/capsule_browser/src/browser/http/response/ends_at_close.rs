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

use super::head::{scan, Framing, Scan};

/* True when the final response's head is whole and gives its body no
length and no chunking, so the body runs until the server closes (RFC 9112
6.3, the last case). Only such a response is whole when the connection
closes; one that declared a length or chunks and closed short is cut. */
pub fn ends_at_close(raw: &[u8]) -> bool {
    matches!(scan(raw), Scan::Head(h) if h.framing == Framing::Close)
}
