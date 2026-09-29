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

//! Over TLS, the next request is sealed on the kept connection: no handshake.

use super::fetch_fixtures::url_of;
use super::fetch_tls_fixtures::{server_records, settled};
use super::fetch_wire::FakeWire;
use super::pool_reuse_tests::OK;
use crate::browser::fetch::pool::{Idle, Pool};
use crate::browser::fetch::tls::decrypt;
use crate::browser::fetch::types::Phase;

#[test]
fn tls_is_kept_and_reused_without_a_handshake() {
    let mut w = FakeWire::at(0);
    w.writable = true;
    let mut pool = Pool::new();
    let (host, handle) = (String::from("10.0.2.2"), 7);
    let tls = Some(settled());
    let (buf, consumed, tx_seq, used) = (Vec::new(), 0, 1, 1);
    let idle = Idle {
        host,
        port: 443,
        https: true,
        handle,
        tls,
        buf,
        consumed,
        tx_seq,
        used,
        since_ms: 0,
    };
    pool.park(&mut w, idle);
    let f = pool.start(&mut w, url_of("https://10.0.2.2/c.png"), None).expect("start");
    assert_eq!((f.handle, f.tx_seq), (handle, 1), "sealed at the connection's next sequence");
    assert_eq!(&w.sent_on(handle)[..3], &[23, 3, 3], "one application record, no hello");
    w.deliver(handle, &server_records(OK, 64, 0));
    let mut done = pool.step(&mut w, 30);
    assert_eq!(done[0].phase, Phase::Decrypt);
    assert_eq!(decrypt(&mut done[0]).as_deref(), Some(OK));
    assert!(w.opened.is_empty());
}
