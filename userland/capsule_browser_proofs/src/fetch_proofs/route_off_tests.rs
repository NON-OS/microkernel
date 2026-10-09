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

//! With Nym or Anyone chosen and its service gone, nothing leaves direct:
//! not the navigation, a stylesheet, an image, a script, a relaunch, nor a
//! request on a direct connection kept from a page before. A route that was
//! off used to mean direct, and the old page's later fetches went out in
//! the clear.

use super::fetch_fixtures::url_of;
use super::fetch_wire::FakeWire;
use super::pool_reuse_tests::OK;
use crate::browser::fetch::open::open;
use crate::browser::fetch::pool::{retain, Pool, Refused};
use crate::browser::net::mixnet::{Network, Routes, Way};

/// Every page the reader may be on with a private network chosen whose
/// service is not running, Anyone's too for .anyone pages.
fn route_off() -> Vec<Routes> {
    let mut all = Vec::new();
    for host in ["example.org", "search.anyone"] {
        all.push(Routes::for_page(host, Network::Nym, 0, 0));
        all.push(Routes::for_page(host, Network::Nym, 0, 42));
        all.push(Routes::for_page(host, Network::Anyone, 41, 0));
        all.push(Routes::for_page(host, Network::Anyone, 0, 0));
    }
    all
}

const TARGETS: [&str; 5] = [
    "https://example.org/",
    "http://cdn.example.org/site.css",
    "https://img.example.net/a.png",
    "http://10.0.2.2:8000/app.js",
    "http://search.anyone/",
];

#[test]
fn no_way_out_is_direct_while_a_private_network_is_chosen() {
    for routes in route_off() {
        for target in TARGETS {
            let host = url_of(target).host;
            assert_ne!(routes.way(&host), Way::Direct, "{routes:?} {target}");
        }
        for chosen in [Network::Nym, Network::Anyone] {
            assert!(!routes.direct(chosen), "net.sockets is refused below the fetch too");
        }
    }
}

#[test]
fn the_navigation_opens_no_socket() {
    for routes in route_off() {
        let mut w = FakeWire::at(0);
        w.routes = Some(routes);
        for target in TARGETS {
            if let Ok(f) = open(&mut w, url_of(target), None) {
                assert!(f.way.proxied(), "{routes:?} {target}: only through a proxy");
            }
        }
        assert!(w.ways.iter().all(|way| way.proxied()), "{routes:?}");
        assert_eq!((w.connects.len(), w.waited.len(), w.resolves), (0, 0, 0), "{routes:?}");
    }
}

#[test]
fn sheets_images_scripts_and_relaunches_open_no_direct_socket() {
    for routes in route_off() {
        let mut w = FakeWire::at(0);
        w.routes = Some(routes);
        let mut pool = Pool::new();
        for target in TARGETS {
            let started = pool.start(&mut w, url_of(target), None).map(|f| f.way);
            let fresh = pool.start_fresh(&mut w, url_of(target), None).map(|f| f.way);
            for got in [started, fresh] {
                match got {
                    Ok(way) => assert!(way.proxied(), "{routes:?} {target}"),
                    Err(Refused::Failed(_) | Refused::Busy) => {}
                }
            }
        }
        assert!(w.ways.iter().all(|way| way.proxied()), "{routes:?}");
        assert_eq!((w.connects.len(), w.resolves), (0, 0), "{routes:?}");
    }
}

#[test]
fn a_direct_connection_kept_from_before_carries_nothing_once_the_route_is_off() {
    let mut w = FakeWire::at(0);
    w.writable = true;
    let mut pool = Pool::new();
    let handle = pool.start(&mut w, url_of("http://10.0.2.2/a.png"), None).expect("direct").handle;
    pool.step(&mut w, 30);
    w.deliver(handle, OK);
    let mut job = pool.step(&mut w, 30).remove(0);
    assert_eq!(job.way, Way::Direct);
    let body = core::mem::take(&mut job.buf);
    let idle = retain(&mut job, &body, 0).expect("kept");
    pool.park(&mut w, idle);
    let on_it = |w: &FakeWire| w.sent.iter().filter(|(h, _)| *h == handle).count();
    let before = on_it(&w);
    for routes in route_off() {
        w.routes = Some(routes);
        let got = pool.start(&mut w, url_of("http://10.0.2.2/b.png"), None).map(|f| f.handle);
        assert_ne!(got, Ok(handle), "{routes:?}: the kept direct connection is not used");
        pool.cancel_live(&mut w);
    }
    assert_eq!(on_it(&w), before, "nothing more went on the direct connection");
    assert!(w.ways[1..].iter().all(|way| way.proxied()), "and no direct socket opened");
}
