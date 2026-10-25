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

//! A joined station holds its PTK and passphrase. The read ops a panel
//! calls — scan, status, link — report the network, never its secret. Over
//! seeded read requests the passphrase never appears in a reply. Connect and
//! disconnect are left out so the session stays up to be probed.

use super::wire::request;
use crate::soft::join::{joined, PASS};
use crate::sweep::rng::each;
use nonos_wifi_core::netif::MAX_RESPONSE;

/// Scan, status, link, and unknowns — never connect or disconnect.
const READ_OPS: [u16; 6] = [3, 4, 5, 0, 6, 0xFFFF];

#[test]
fn no_read_reply_carries_the_passphrase() {
    let mut st = joined();
    each(|rng| {
        let req = request(rng, &READ_OPS);
        let mut out = vec![0u8; MAX_RESPONSE];
        let n = st.handle(&req, &mut out);
        let leaked = out[..n].windows(PASS.len()).any(|w| w == PASS);
        assert!(!leaked, "the passphrase left in a control reply");
    });
}
