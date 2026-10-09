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

//! The route a connection takes: the one the person chose, or none. Every
//! answer the policy store could give, with each anonymity network up or
//! down, and nothing ever leads to a direct connection but Direct itself.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::pick::{pick, Route, ANYONE_DOWN, NYM_DOWN, UNREAD};

const SOCKS: u32 = 4908;
const ANON: u32 = 4911;

fn every_default() -> impl Iterator<Item = Option<u8>> {
    core::iter::once(None).chain((0..=u8::MAX).map(Some))
}

const UP_DOWN: [(u32, u32); 4] = [(0, 0), (SOCKS, 0), (0, ANON), (SOCKS, ANON)];

#[test]
fn the_whole_table() {
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            let want = match default {
                Some(DIRECT) => Route::Direct,
                Some(ANYONE) if anon != 0 => Route::Anon(anon),
                Some(ANYONE) => Route::Down(ANYONE_DOWN),
                _ if nym != 0 => Route::Nym(nym),
                Some(_) => Route::Down(NYM_DOWN),
                None => Route::Down(UNREAD),
            };
            assert_eq!(pick(default, nym, anon), want, "{default:?} nym={nym} anon={anon}");
        }
    }
}

#[test]
fn nothing_but_direct_itself_goes_direct() {
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            if pick(default, nym, anon) == Route::Direct {
                assert_eq!(default, Some(DIRECT), "nym={nym} anon={anon}");
            }
        }
    }
}

#[test]
fn a_network_that_is_down_is_never_replaced_by_another() {
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            match pick(default, nym, anon) {
                Route::Anon(port) => assert_eq!((default, port), (Some(ANYONE), ANON)),
                Route::Nym(port) => {
                    assert!(default != Some(ANYONE) && default != Some(DIRECT));
                    assert_eq!(port, SOCKS);
                }
                Route::Down(_) => match default {
                    Some(DIRECT) => panic!("Direct is never down"),
                    Some(ANYONE) => assert_eq!(anon, 0),
                    _ => assert_eq!(nym, 0),
                },
                Route::Direct => {}
            }
        }
    }
}

#[test]
fn an_unreadable_or_unknown_default_is_the_mixnet() {
    assert_eq!(pick(None, SOCKS, ANON), Route::Nym(SOCKS));
    assert_eq!(pick(Some(3), SOCKS, ANON), Route::Nym(SOCKS));
    assert_eq!(pick(Some(u8::MAX), 0, ANON), Route::Down(NYM_DOWN));
    assert_eq!(pick(None, 0, ANON), Route::Down(UNREAD));
    assert_eq!(pick(Some(NYM), SOCKS, 0), Route::Nym(SOCKS));
}

#[test]
fn how_a_route_reads_and_waits() {
    assert_eq!(Route::Nym(SOCKS).name(), "through the Nym mixnet");
    assert_eq!(Route::Anon(ANON).name(), "through the Anyone onion network");
    assert_eq!(Route::Direct.name(), "over a direct connection");
    assert_eq!(Route::Down(NYM_DOWN).name(), NYM_DOWN);
    assert_eq!(Route::Nym(SOCKS).label(), "Nym");
    assert_eq!(Route::Anon(ANON).label(), "Anyone");
    assert_eq!(Route::Direct.label(), "Direct");
    assert_eq!(Route::Down(UNREAD).label(), "no route");
    /* A direct reader keeps the windows it had; an anonymous one waits. */
    assert_eq!(Route::Direct.patience_ms(), 0);
    assert!(Route::Nym(SOCKS).patience_ms() >= 30_000);
    assert!(Route::Anon(ANON).patience_ms() >= 15_000);
    assert!(Route::Nym(SOCKS).is_anonymous() && Route::Anon(ANON).is_anonymous());
    assert!(!Route::Direct.is_anonymous() && !Route::Down(UNREAD).is_anonymous());
}

/* An .anyone service is reached only inside the Anyone network. */
const SERVICE: &str = "mforujillfk4w5h2zqgxdzendn3qb2zes4r2h57ownojshhmtcdzgdyd.anyone";

#[test]
fn an_anyone_service_goes_through_net_anon_whatever_the_default() {
    use crate::pick::for_host;
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            let chosen = pick(default, nym, anon);
            let want =
                if anon != 0 { Route::Anon(anon) } else { Route::Down(crate::pick::ANYONE_NEEDED) };
            assert_eq!(
                for_host(chosen, SERVICE, anon),
                want,
                "default {default:?} nym {nym} anon {anon}"
            );
        }
    }
}

#[test]
fn an_anyone_service_never_leaves_directly() {
    use crate::pick::for_host;
    for anon in [0, ANON] {
        assert_ne!(for_host(Route::Direct, SERVICE, anon), Route::Direct);
    }
}

#[test]
fn every_other_host_keeps_the_chosen_route() {
    use crate::pick::for_host;
    let hosts = [
        "sepolia.gateway.tenderly.co",
        "example.onion",
        "anyone",
        ".anyone",
        "x.anyone.example",
        "anyoneanyone",
        "notanyone",
    ];
    for default in every_default() {
        for (nym, anon) in UP_DOWN {
            let chosen = pick(default, nym, anon);
            for host in hosts {
                assert_eq!(for_host(chosen, host, anon), chosen, "{host}");
            }
        }
    }
}

#[test]
fn the_service_name_is_read_in_any_case_and_with_the_root_dot() {
    use crate::pick::is_anyone;
    assert!(is_anyone(SERVICE));
    assert!(is_anyone("WWW.EXAMPLE.ANYONE"));
    assert!(is_anyone("x.Anyone."));
    assert!(!is_anyone(".anyone"));
    assert!(!is_anyone("anyone"));
    // Extra root dots stay inside the Anyone network, where net.anon refuses
    // the name, rather than going out to an exit.
    assert!(is_anyone("x.anyone.."));
    assert!(!is_anyone("x.onion"));
    assert!(!is_anyone("x.anyoné"));
}

#[test]
fn a_full_address_is_not_a_short_name() {
    let full = "iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone";
    assert!(!crate::pick::is_short_anyone(full));
    assert!(!crate::pick::is_short_anyone("iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.ANYONE.."));
}

#[test]
fn a_short_name_is_one() {
    assert!(crate::pick::is_short_anyone("shield.anyone"));
    assert!(crate::pick::is_short_anyone("shield.anyone."));
    assert!(!crate::pick::is_short_anyone("shield.example"));
    assert!(!crate::pick::is_short_anyone(".anyone"));
}

#[test]
fn a_short_name_refusal_says_what_to_do() {
    use crate::refusal::Proxy;
    assert!(Proxy::AnyoneName.refused(4).contains("signed list"));
    assert_eq!(Proxy::AnyoneName.refused(6), Proxy::AnyoneService.refused(6));
}

/*
 * An install's download goes over Anyone whatever the default network is,
 * and is no route, never another network, while net.anon has not
 * registered.
 */
#[test]
fn installs_take_anyone_whatever_the_default() {
    use crate::pick::{install_route, ANYONE_INSTALLS_DOWN};
    assert_eq!(install_route(9150), Route::Anon(9150));
    assert_eq!(install_route(0), Route::Down(ANYONE_INSTALLS_DOWN));
}
