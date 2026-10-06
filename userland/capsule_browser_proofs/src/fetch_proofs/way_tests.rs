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

//! Each connection leaves the way its own host needs: an .anyone address
//! through net.anon whatever the reader chose, or not at all; everything of
//! an .anyone page through Anyone; direct only on a page whose network is
//! Direct. On the old browser every request took the reader's network, so
//! an .anyone address went to the Nym mixnet, or to net.dns in the clear.

use super::fetch_fixtures::{step, url_of};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::open::open;
use crate::browser::fetch::types::Phase;
use crate::browser::net::mixnet::{Network, Routes, Way};
use nonos_route_link::{Proxy, ANYONE_NEEDED};

const NYM: u32 = 41;
const ANON: u32 = 42;
const ONION: &str = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd.anyone";
const ALL: [Network; 3] = [Network::Direct, Network::Nym, Network::Anyone];

fn page(host: &str, chosen: Network, nym: u32, anon: u32) -> Routes {
    Routes::for_page(host, chosen, nym, anon)
}

#[test]
fn an_anyone_address_goes_through_net_anon_whatever_the_choice() {
    for chosen in ALL {
        let r = page("example.org", chosen, NYM, ANON);
        let service = Way::Proxy { net: Network::Anyone, port: ANON, proxy: Proxy::AnyoneService };
        assert_eq!(r.way(ONION), service, "{chosen:?}");
        let name = Way::Proxy { net: Network::Anyone, port: ANON, proxy: Proxy::AnyoneName };
        assert_eq!(r.way("search.anyone"), name, "{chosen:?}: a short name");
    }
}

#[test]
fn an_anyone_address_with_net_anon_down_is_refused_never_sent_elsewhere() {
    for chosen in ALL {
        let r = page("example.org", chosen, NYM, 0);
        assert_eq!(r.way(ONION), Way::Refused(ANYONE_NEEDED), "{chosen:?}");
        assert_eq!(r.way("search.anyone."), Way::Refused(ANYONE_NEEDED), "{chosen:?}");
    }
}

#[test]
fn everything_an_anyone_page_asks_for_leaves_through_anyone() {
    for chosen in ALL {
        let r = page(ONION, chosen, NYM, ANON);
        assert_eq!(r.page, Network::Anyone);
        let exit = Way::Proxy { net: Network::Anyone, port: ANON, proxy: Proxy::Anyone };
        assert_eq!(r.way("cdn.example.org"), exit, "{chosen:?}: its clearnet image too");
        assert!(!r.direct(chosen), "{chosen:?}: nothing direct from an onion page");
    }
}

#[test]
fn a_clearnet_page_keeps_the_readers_network() {
    let nym = Way::Proxy { net: Network::Nym, port: NYM, proxy: Proxy::Nym };
    assert_eq!(page("a.org", Network::Nym, NYM, ANON).way("b.org"), nym);
    let anyone = Way::Proxy { net: Network::Anyone, port: ANON, proxy: Proxy::Anyone };
    assert_eq!(page("a.org", Network::Anyone, NYM, ANON).way("b.org"), anyone);
    assert_eq!(page("a.org", Network::Direct, NYM, ANON).way("b.org"), Way::Direct);
}

#[test]
fn a_private_choice_whose_service_is_down_is_refused() {
    let r = page("a.org", Network::Nym, 0, ANON);
    assert_eq!(r.way("b.org"), Way::Refused(Network::Nym.absent()));
    let r = page("a.org", Network::Anyone, NYM, 0);
    assert_eq!(r.way("b.org"), Way::Refused(Network::Anyone.absent()));
}

#[test]
fn direct_is_taken_only_on_a_direct_page_of_a_reader_who_chose_it() {
    for host in ["a.org", ONION, "search.anyone"] {
        for chosen in ALL {
            for (nym, anon) in [(0, 0), (NYM, 0), (0, ANON), (NYM, ANON)] {
                let r = page(host, chosen, nym, anon);
                for target in ["b.org", ONION, "x.anyone", "10.0.2.2"] {
                    if r.way(target) == Way::Direct {
                        assert_eq!(chosen, Network::Direct, "{host} {target}");
                        assert!(!target.ends_with(".anyone") && !host.ends_with(".anyone"));
                    }
                }
                assert_eq!(r.direct(chosen), chosen == Network::Direct && !host.ends_with(".anyone"));
                assert!(!r.direct(Network::Nym), "a reader who moved on is not sent direct");
            }
        }
    }
}

#[test]
fn a_refusal_is_worded_for_the_way_that_was_refused() {
    let nym = Way::Proxy { net: Network::Nym, port: NYM, proxy: Proxy::Nym };
    assert_eq!(nym.refused(4), "The Nym mixnet has no exit for this destination.");
    let service = Way::Proxy { net: Network::Anyone, port: ANON, proxy: Proxy::AnyoneService };
    assert_eq!(service.refused(4), Proxy::AnyoneService.refused(4));
    let name = Way::Proxy { net: Network::Anyone, port: ANON, proxy: Proxy::AnyoneName };
    assert!(name.refused(4).starts_with("that short .anyone name is not in the signed list"));
}

#[test]
fn an_anyone_address_is_refused_before_a_socket_opens() {
    let mut w = FakeWire::at(0);
    w.routes = Some(page("example.org", Network::Direct, NYM, 0));
    let got = open(&mut w, url_of("http://search.anyone/"), None);
    assert_eq!(got.err(), Some(ANYONE_NEEDED));
    assert!(w.ways.is_empty() && w.opened.is_empty(), "nothing opened, nothing resolved");
    assert_eq!(w.resolves, 0);
}

#[test]
fn a_refused_short_name_says_why_in_route_links_words() {
    let mut w = FakeWire::at(0);
    w.routes = Some(page("search.anyone", Network::Nym, NYM, ANON));
    let mut f = open(&mut w, url_of("http://search.anyone/"), None).expect("open");
    assert_eq!(w.ways, vec![Way::Proxy { net: Network::Anyone, port: ANON, proxy: Proxy::AnyoneName }]);
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0]);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::SocksConnect);
    w.deliver(f.handle, &[5, 4, 0, 1, 0, 0, 0, 0, 0, 0]);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(Proxy::AnyoneName.refused(4)));
}

#[test]
fn an_anyone_fetch_keeps_anyones_budget_on_a_nym_page() {
    let mut w = FakeWire::at(0);
    w.routes = Some(page("example.org", Network::Nym, NYM, ANON));
    let mut f = open(&mut w, url_of(&format!("http://{ONION}/")), None).expect("open");
    assert_eq!(f.way.network(), Network::Anyone);
    step(&mut w, &mut f);
    w.advance(30_001);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("timed out"));
}
