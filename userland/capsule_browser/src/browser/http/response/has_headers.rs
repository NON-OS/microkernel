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

use super::head::{scan, Scan};

/* True once the final response's header section has arrived (interim
1xx responses do not count), or bytes arrived that are no response. */
pub fn has_headers(raw: &[u8]) -> bool {
    !matches!(scan(raw), Scan::More)
}

/* True unless the response is framed and its server keeps the
connection: a `Connection: close`, an HTTP/1.0 response without
keep-alive, or a head that does not scan. */
pub fn wants_close(raw: &[u8]) -> bool {
    match scan(raw) {
        Scan::Head(h) => h.close,
        _ => true,
    }
}
