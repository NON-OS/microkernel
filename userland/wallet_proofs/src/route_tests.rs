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

//! The network the wallet's requests go over, as its screens say it, and
//! the reader for answers through an anonymity network: whole flights,
//! early stops, and every wait bounded by time rather than by a count.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::pick::{pick, Route, NYM_DOWN};
use crate::wallet::net::route_text::{route_part, route_value};
use crate::wallet::net::routed_read::{read_routed, Bounds, Source};

#[test]
fn the_status_line_names_the_route_the_requests_took() {
    assert_eq!(route_part(None), None, "nothing is claimed before a check");
    assert_eq!(route_part(Some(pick(Some(NYM), 7, 9))), Some("Nym"));
    assert_eq!(route_part(Some(pick(Some(ANYONE), 7, 9))), Some("Anyone"));
    /* Direct reads as Direct, even with the mixnet running: it used to say
     * "Nym" whenever net.nym answered a health check. */
    assert_eq!(route_part(Some(pick(Some(DIRECT), 7, 9))), Some("Direct"));
    assert_eq!(route_part(Some(pick(Some(NYM), 0, 9))), Some("no route"));
}

#[test]
fn the_route_row_says_how_the_rpc_host_is_reached() {
    assert_eq!(route_value(Some(Route::Nym(7))), "public RPC via Nym");
    assert_eq!(route_value(Some(Route::Anon(9))), "public RPC via Anyone");
    assert_eq!(route_value(Some(Route::Direct)), "public RPC, direct");
    assert_eq!(route_value(Some(Route::Down(NYM_DOWN))), "no route");
    assert_eq!(route_value(None), "public RPC");
}

/// A stream through a proxy, scripted: each read either brings `bytes`
/// after `after_ms`, or nothing until the wait runs out; `None` in the
/// script is the far end finishing.
struct Script {
    now: i64,
    steps: Vec<Option<(i64, Vec<u8>)>>,
    waits: Vec<u64>,
    broke: bool,
}

impl Script {
    fn new(steps: Vec<Option<(i64, Vec<u8>)>>) -> Script {
        Script { now: 0, steps, waits: Vec::new(), broke: false }
    }
}

impl Source for Script {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        self.waits.push(wait_ms);
        if self.broke {
            return Err("broke");
        }
        if self.steps.is_empty() {
            self.now += wait_ms as i64;
            return Ok(0);
        }
        match self.steps.remove(0) {
            None => Ok(0),
            Some((after, bytes)) if after <= wait_ms as i64 => {
                self.now += after;
                let n = bytes.len().min(into.len());
                into[..n].copy_from_slice(&bytes[..n]);
                Ok(n)
            }
            Some(later) => {
                self.now += wait_ms as i64;
                self.steps.insert(0, Some((later.0 - wait_ms as i64, later.1)));
                Ok(0)
            }
        }
    }

    fn now_ms(&self) -> i64 {
        self.now
    }
}

const BOUNDS: Bounds = Bounds { first_ms: 60_000, quiet_ms: 8_000, total_ms: 90_000, max: 64 };

#[test]
fn a_reply_that_comes_late_and_in_pieces_is_read_whole() {
    let mut s = Script::new(vec![
        Some((20_000, b"HTTP/1.1 ".to_vec())),
        Some((3_000, b"200 OK\r\n".to_vec())),
        Some((7_000, b"\r\n{}".to_vec())),
        None,
    ]);
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), Some(b"HTTP/1.1 200 OK\r\n\r\n{}".to_vec()));
    /* The first wait is the route's patience, the rest the quiet window. */
    assert_eq!(s.waits[0], 60_000);
    assert!(s.waits[1..].iter().all(|&w| w <= 8_000), "{:?}", s.waits);
}

#[test]
fn a_flight_stops_as_soon_as_it_is_whole() {
    let mut s = Script::new(vec![
        Some((1_000, b"ab".to_vec())),
        Some((1_000, b"cd".to_vec())),
        Some((1_000, b"ef".to_vec())),
    ]);
    let got = read_routed(&mut s, BOUNDS, |b| b.len() >= 4);
    assert_eq!(got, Some(b"abcd".to_vec()));
    assert_eq!(s.steps.len(), 1, "nothing read past the end of the flight");
}

#[test]
fn a_pause_longer_than_the_quiet_window_ends_the_answer() {
    let mut s = Script::new(vec![Some((1_000, b"ab".to_vec())), Some((9_000, b"cd".to_vec()))]);
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), Some(b"ab".to_vec()));
}

#[test]
fn nothing_at_all_is_an_error_after_the_first_wait() {
    let mut s = Script::new(vec![]);
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), None);
    assert_eq!(s.now, 60_000);
    let mut closed = Script::new(vec![None]);
    assert_eq!(read_routed(&mut closed, BOUNDS, |_| false), None);
}

#[test]
fn more_than_a_flight_holds_is_refused() {
    let mut s = Script::new(vec![Some((10, vec![1u8; 60])), Some((10, vec![2u8; 10]))]);
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), None);
}

#[test]
fn a_drip_cannot_hold_the_wallet_past_the_total() {
    let steps = (0..1_000).map(|_| Some((7_999, vec![1u8]))).collect();
    let mut s = Script::new(steps);
    let bounds = Bounds { max: 4_096, ..BOUNDS };
    let got = read_routed(&mut s, bounds, |_| false);
    assert!(got.is_some());
    assert!(s.now <= 90_000 + 8_000, "stopped by the total: {}", s.now);
}

/// A stream that breaks once its script has run out.
struct Breaks(Script);

impl Source for Breaks {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        self.0.broke = self.0.steps.is_empty();
        self.0.read_wait(into, wait_ms)
    }

    fn now_ms(&self) -> i64 {
        self.0.now
    }
}

#[test]
fn a_stream_that_breaks_keeps_what_came_before_and_nothing_else() {
    let mut s = Breaks(Script::new(vec![Some((10, b"part".to_vec()))]));
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), Some(b"part".to_vec()));
    let mut at_once = Breaks(Script::new(vec![]));
    assert_eq!(read_routed(&mut at_once, BOUNDS, |_| false), None);
}

/* The wallet's RPC never leaves Direct: a Direct default goes over Nym,
 * else Anyone, and with neither there is no route at all. */
#[test]
fn the_wallet_reads_the_chain_only_over_an_anonymity_network() {
    use crate::pick::{private_only, PRIVATE_DOWN};
    assert_eq!(private_only(Route::Direct, 7, 9), Route::Nym(7));
    assert_eq!(private_only(Route::Direct, 0, 9), Route::Anon(9));
    assert_eq!(private_only(Route::Direct, 0, 0), Route::Down(PRIVATE_DOWN));
    assert_eq!(private_only(Route::Anon(9), 7, 9), Route::Anon(9));
    assert_eq!(private_only(Route::Nym(7), 0, 9), Route::Nym(7));
    assert_eq!(private_only(Route::Down(NYM_DOWN), 0, 0), Route::Down(NYM_DOWN));
    for default in [Some(DIRECT), Some(NYM), Some(ANYONE), None] {
        for (nym, anon) in [(0, 0), (7, 0), (0, 9), (7, 9)] {
            let r = private_only(pick(default, nym, anon), nym, anon);
            assert_ne!(r, Route::Direct, "{default:?} {nym} {anon}");
        }
    }
}

/* A host that fails leaves its place to the next on its network, once,
 * however many requests failed on it, and the other network keeps its own. */
#[test]
fn a_failed_rpc_host_hands_over_to_the_next_on_its_network() {
    use crate::wallet::chain::{pick, rpc_failed, rpc_host, MAINNET, SEPOLIA};
    let _held = crate::fetch_tests::hold();
    /* Start from the first host, whatever an earlier test left. */
    pick(false);
    while rpc_host() != MAINNET.rpcs[0] {
        rpc_failed(rpc_host());
    }
    pick(true);
    while rpc_host() != SEPOLIA.rpcs[0] {
        rpc_failed(rpc_host());
    }
    pick(false);
    let first = rpc_host();
    assert_eq!(first, MAINNET.rpcs[0]);
    assert!(rpc_failed(first));
    assert!(!rpc_failed(first), "a second failure on the old host moves nothing");
    assert_eq!(rpc_host(), MAINNET.rpcs[1]);
    pick(true);
    assert_eq!(rpc_host(), SEPOLIA.rpcs[0]);
    pick(false);
    for _ in 0..MAINNET.rpcs.len() - 1 {
        assert!(rpc_failed(rpc_host()));
    }
    assert_eq!(rpc_host(), MAINNET.rpcs[0], "round to the first again");
}

/* What Home and Settings say of each network's last read. */
#[test]
fn the_last_read_names_its_block_host_route_and_age() {
    use crate::last_read::{read_line, LastRead};
    let r = LastRead {
        block: 23_456_789,
        host: "ethereum-rpc.publicnode.com",
        route: "Nym",
        at_ms: 1_000,
    };
    assert_eq!(
        read_line(Some(r), 13_000),
        "block 23,456,789 from ethereum-rpc.publicnode.com over Nym, 12 s ago"
    );
    assert_eq!(
        read_line(Some(r), 1_000 + 125_000),
        "block 23,456,789 from ethereum-rpc.publicnode.com over Nym, 2 min ago"
    );
    assert_eq!(read_line(None, 0), "not read since this boot");
    let small = LastRead { block: 999, ..r };
    assert!(read_line(Some(small), 1_000).starts_with("block 999 from"));
}
