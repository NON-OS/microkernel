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

use alloc::vec::Vec;

use crate::browser::fetch::types::Fetch;

/*
 * The response was decrypted from its first record on every read, and on a
 * kept connection from the connection's first record: the n-th image opened
 * every record before it again. The reader keeps its place, so each call
 * opens only the records that completed since the last one.
 */
/// This response's plaintext so far, or `None` before the handshake has
/// verified: there is nothing to read without its keys, and no reason to
/// verify again to find out.
pub fn plain(f: &mut Fetch) -> Option<&[u8]> {
    let Fetch { tls, buf, rx_consumed, .. } = f;
    let tls = tls.as_mut()?;
    let app = tls.server_app.as_ref()?;
    tls.reader.feed(app, buf);
    Some(tls.reader.plaintext().get(*rx_consumed..).unwrap_or(&[]))
}

/// The same, copied out for code that keeps the response.
pub fn decrypt(f: &mut Fetch) -> Option<Vec<u8>> {
    plain(f).map(<[u8]>::to_vec)
}

/// A record failed to open, so nothing after it is believed.
pub fn broken(f: &Fetch) -> bool {
    f.tls.as_ref().is_some_and(|tls| tls.reader.is_broken())
}
