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

//! Several fetches at once, within a limit per host and in all.

use super::fetch_fixtures::url_of;
use super::fetch_wire::FakeWire;
use crate::browser::fetch::pool::{Pool, Refused};

fn start(pool: &mut Pool, w: &mut FakeWire, target: &str) -> Result<(), Refused> {
    pool.start(w, url_of(target), None).map(|_| ())
}

#[test]
fn six_to_one_host_and_eight_in_all() {
    let mut w = FakeWire::at(0);
    let mut pool = Pool::new();
    for i in 0..6 {
        assert!(start(&mut pool, &mut w, &format!("http://10.0.2.2/p{i}.png")).is_ok());
    }
    let seventh = start(&mut pool, &mut w, "http://10.0.2.2/p6.png");
    assert_eq!(seventh, Err(Refused::Busy), "the seventh to one host waits");
    assert!(start(&mut pool, &mut w, "http://10.0.2.3/a.css").is_ok(), "another host starts");
    assert!(start(&mut pool, &mut w, "http://10.0.2.4/b.js").is_ok());
    let ninth = start(&mut pool, &mut w, "http://10.0.2.5/c.png");
    assert_eq!(ninth, Err(Refused::Busy), "the ninth in all waits");
    assert_eq!((pool.live.len(), w.opened.len()), (8, 8));
    assert!(pool.busy());
}

#[test]
fn the_ports_of_one_host_are_separate_hosts() {
    let mut w = FakeWire::at(0);
    let mut pool = Pool::new();
    for i in 0..6 {
        assert!(start(&mut pool, &mut w, &format!("http://10.0.2.2:8088/{i}")).is_ok());
    }
    assert!(start(&mut pool, &mut w, "http://10.0.2.2:8089/").is_ok());
}

#[test]
fn the_mixnet_carries_one_conversation() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut pool = Pool::new();
    assert!(start(&mut pool, &mut w, "https://a.test/").is_ok());
    assert_eq!(start(&mut pool, &mut w, "https://b.test/"), Err(Refused::Busy));
}
