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

//! A second request to a host goes out on the connection the first kept.

use super::fetch_fixtures::url_of;
use super::fetch_wire::FakeWire;
use crate::browser::fetch::pool::{retain, Pool};
use crate::browser::fetch::types::Phase;

pub const OK: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok";

#[test]
fn plain_http_is_kept_and_reused_without_a_connect() {
    let mut w = FakeWire::at(0);
    w.writable = true;
    let mut pool = Pool::new();
    let handle = pool.start(&mut w, url_of("http://10.0.2.2/a.png"), None).expect("start").handle;
    assert!(pool.step(&mut w, 30).is_empty(), "connected and asked");
    w.deliver(handle, OK);
    let mut done = pool.step(&mut w, 30);
    assert_eq!(done.len(), 1, "ended in the call that read its last byte");
    let mut job = done.remove(0);
    let body = core::mem::take(&mut job.buf);
    let idle = retain(&mut job, &body, 0).expect("framed, and not asked to close");
    pool.park(&mut w, idle);
    let second = pool.start(&mut w, url_of("http://10.0.2.2/b.png"), None).expect("start");
    assert_eq!((second.handle, second.phase, second.keep_uses), (handle, Phase::ReadBody, 1));
    assert_eq!((w.opened.len(), w.connects.len()), (1, 1), "no second connection");
    assert!(w.sent_on(handle).ends_with(b"\r\n\r\n"));
    assert_eq!(w.sent.iter().filter(|(h, b)| *h == handle && b.starts_with(b"GET ")).count(), 2);
}
