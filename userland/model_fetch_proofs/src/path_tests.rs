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

/*
 * The path a model download takes and what is said of it: never a network
 * the person did not choose; direct only when they asked for it for that
 * download; an estimate and the offer before a large download through Nym
 * or Anyone; the time left only once it can be said honestly; where a
 * download stands as the store is answered; the store's `d` read by init
 * for a tier only; and the room refusal for a live boot and a disk.
 */

use crate::errno::no_room;
use crate::eta::{eta, MEASURED_BYTES, MEASURED_MS};
use crate::init_direct::{split, DIRECT};
use crate::net::pick::{pick, Route, NYM_DOWN};
use crate::offer::{offer_line, route_for};
use crate::path::{duration, offers_direct, ASK_OVER, NYM_RATE};
use crate::status_wire::{Status, ANYONE, BROKEN, CHECKING, FETCHING, LEN, NYM};
use crate::store::route_offer::{direct_offered, path_line, progress_line, store_offer};
use nonos_policy_proto::route;

const GB: u64 = 1_000_000_000;

fn every_route() -> [Route; 5] {
    [Route::Nym(9050), Route::Anon(9150), Route::Direct, Route::Down(NYM_DOWN), Route::Down("")]
}

/* The chosen route is kept unless the person asked for direct; nothing else turns a route direct. */
#[test]
fn direct_only_when_asked_for_this_download() {
    for r in every_route() {
        assert_eq!(route_for(r, false), r);
        assert_eq!(route_for(r, true), Route::Direct);
    }
    /* A default of Nym that is down stays down: no download, never a quiet direct one. */
    let down = pick(Some(route::NYM), 0, 0);
    assert!(matches!(route_for(down, false), Route::Down(_)));
}

#[test]
fn the_offer_is_made_only_past_the_threshold_through_an_anonymity_network() {
    for left in [GB / 2, ASK_OVER] {
        for r in every_route() {
            assert!(!offers_direct(r, left), "{r:?} {left}");
            assert_eq!(offer_line(r, left, "qwen3-8b"), None);
            assert_eq!(store_offer(r, left), None);
        }
    }
    for r in [Route::Nym(1), Route::Anon(1)] {
        assert!(offers_direct(r, ASK_OVER + 1));
    }
    for r in [Route::Direct, Route::Down(NYM_DOWN)] {
        assert!(!offers_direct(r, 100 * GB));
        assert_eq!(offer_line(r, 100 * GB, "max"), None);
    }
}

#[test]
fn the_offer_says_the_path_the_estimate_and_what_direct_costs() {
    let line = offer_line(Route::Nym(1), 5_000_000_000, "qwen3-8b").unwrap();
    assert!(line.starts_with("5.00 GB through the Nym mixnet takes about 5 h 33 min"), "{line}");
    assert!(line.contains("at the 250 KB/s assumed for it"), "{line}");
    assert!(line.contains("`qwen get --direct qwen3-8b`"), "{line}");
    assert!(line.contains("the mirror sees this machine's address"), "{line}");
    let anyone = offer_line(Route::Anon(1), 5_000_000_000, "qwen3-8b").unwrap();
    assert!(anyone.contains("through the Anyone onion network takes about 1 h 23 min"), "{anyone}");
    let store = store_offer(Route::Nym(1), 5_000_000_000).unwrap();
    assert!(store.starts_with("Through the Nym mixnet: about 5 h 33 min"), "{store}");
    assert!(store.contains("Press d to download direct") && store.contains("mirror sees"), "{store}");
    assert_eq!(5_000_000_000 / NYM_RATE, 20_000);
}

#[test]
fn d_is_offered_only_before_install_and_only_where_the_offer_is() {
    assert!(direct_offered(Route::Nym(1), 2 * GB, true, None));
    assert!(!direct_offered(Route::Nym(1), 2 * GB, false, None));
    assert!(!direct_offered(Route::Nym(1), GB / 2, true, None));
    assert!(!direct_offered(Route::Direct, 20 * GB, true, None));
    /* After the network or its exits did not answer, whatever the size; never on Direct. */
    for why in [28u8, 29, 31, 32] {
        assert!(direct_offered(Route::Nym(1), GB / 2, false, Some(why)), "{why}");
        assert!(direct_offered(Route::Anon(1), GB / 2, false, Some(why)), "{why}");
        assert!(!direct_offered(Route::Direct, GB / 2, false, Some(why)), "{why}");
    }
    /* Anyone still starting is Anyone: before Install, the offer stands. */
    assert!(direct_offered(Route::Down(NYM_DOWN), 2 * GB, true, None));
    assert!(direct_offered(Route::Down(NYM_DOWN), GB / 2, false, Some(33)));
    for why in [15u8, 16, 12] {
        assert!(!direct_offered(Route::Nym(1), GB / 2, false, Some(why)), "{why}");
    }
}

#[test]
fn durations_read_as_a_person_says_them() {
    assert_eq!(duration(0), "under a minute");
    assert_eq!(duration(59), "under a minute");
    assert_eq!(duration(60), "about 1 min");
    assert_eq!(duration(61), "about 2 min");
    assert_eq!(duration(3_599), "about 60 min");
    assert_eq!(duration(3_600), "about 1 h 0 min");
    assert_eq!(duration(20_000), "about 5 h 33 min");
}

/* The time left is said only once 10 s and 4 MiB have been measured. */
#[test]
fn the_time_left_waits_until_it_can_be_said_honestly() {
    assert_eq!(eta(GB, MEASURED_BYTES, MEASURED_MS - 1), None);
    assert_eq!(eta(GB, MEASURED_BYTES - 1, 60_000), None);
    assert_eq!(eta(GB, 0, 0), None);
    /* 10 MB in 10 s is 1 MB/s: 1 GB is 1000 s. */
    assert_eq!(eta(GB, 10_000_000, 10_000), Some(1_000));
    assert_eq!(eta(0, 10_000_000, 10_000), Some(0));
}

#[test]
fn the_status_answer_reads_back_whole_and_refuses_anything_else() {
    let s = Status { stage: FETCHING, route: NYM, total: 5 * GB, done: GB, rate: 250_000, try_n: 0, tries: 0 };
    let b = s.encode();
    assert_eq!(b.len(), LEN);
    assert_eq!(Status::decode(&b), Some(s));
    assert_eq!(Status::decode(&b[..LEN - 1]), None);
    let mut bad = b;
    bad[0] = b'X';
    assert_eq!(Status::decode(&bad), None);
    for stage in [0u8, 6, 255] {
        let mut odd = b;
        odd[4] = stage;
        assert_eq!(Status::decode(&odd), None, "{stage}");
    }
    let over = Status { done: 6 * GB, ..s }.encode();
    assert_eq!(Status::decode(&over), None);
}

#[test]
fn the_card_says_where_a_download_stands() {
    let s = Status {
        stage: FETCHING,
        route: NYM,
        total: 4_680_000_000,
        done: 1_200_000_000,
        rate: 0,
        try_n: 0,
        tries: 0,
    };
    assert_eq!(progress_line(&s), "1.20 GB of 4.68 GB through the Nym mixnet, measuring the rate");
    let s = Status { rate: 2_000_000, route: ANYONE, ..s };
    assert_eq!(
        progress_line(&s),
        "1.20 GB of 4.68 GB over Anyone, 2 MB/s, about 29 min left"
    );
    let broken = Status { stage: BROKEN, ..s };
    assert_eq!(progress_line(&broken), "The connection dropped at 1.20 GB of 4.68 GB; it resumes from there");
    let waiting = Status { stage: BROKEN, route: NYM, try_n: 2, tries: 6, ..s };
    assert_eq!(
        progress_line(&waiting),
        "Nym exit did not answer; trying the next one (2 of 6), from 1.20 GB of 4.68 GB"
    );
    let b = waiting.encode();
    assert_eq!(Status::decode(&b), Some(waiting));
    let checking = Status { stage: CHECKING, done: s.total, ..s };
    assert!(progress_line(&checking).contains("checking its SHA-256 against the signed pin"));
    assert_eq!(path_line(Route::Anon(1)), "Downloads over Anyone");
    assert!(path_line(Route::Down(NYM_DOWN)).starts_with("Downloads over Anyone, which is still starting"));
}

/* The store's `d` reaches the fetcher only for a tier, with the default release. */
#[test]
fn init_reads_at_direct_only_for_a_tier() {
    assert_eq!(DIRECT, "@direct");
    assert_eq!(split("linux.qwen-qwen3-8b", "@direct"), ("", true));
    assert_eq!(split("linux.qwen-qwen3-8b", ""), ("", false));
    assert_eq!(split("linux.qwen-qwen3-8b", "1.0"), ("1.0", false));
    assert_eq!(split("linux.jq", "@direct"), ("@direct", false));
    assert_eq!(split("nonos.terminal", "@direct"), ("@direct", false));
}

#[test]
fn no_room_names_memory_on_a_live_boot_and_the_disk_otherwise() {
    let live = no_room(true, 2_490_000_000);
    assert!(live.contains("holds its data volume in memory") && live.contains("2.49 GB"), "{live}");
    assert!(live.ends_with("(ENOSPC)"), "{live}");
    let disk = no_room(false, 2_490_000_000);
    assert!(disk.starts_with("the data volume on the disk has no room for the 2.49 GB"), "{disk}");
}

/*
 * Installs download over Anyone, whatever network the browser uses; direct
 * only when asked for that download; and the route is said as such.
 */
#[test]
fn installs_go_over_anyone_unless_direct_is_asked() {
    use crate::net::pick::{install_route, ANYONE_INSTALLS_DOWN};
    use crate::offer::route_said;
    assert_eq!(install_route(9150), Route::Anon(9150));
    assert_eq!(install_route(0), Route::Down(ANYONE_INSTALLS_DOWN));
    assert_eq!(route_for(install_route(9150), false), Route::Anon(9150));
    assert_eq!(route_for(install_route(9150), true), Route::Direct);
    assert_eq!(route_said(Route::Anon(9150)), "downloading over Anyone");
    assert_eq!(route_said(Route::Direct), "downloading over a direct connection, as asked");
}

/* While Anyone builds its circuit, the card says which step it is on. */
#[test]
fn the_card_says_anyone_is_building_its_circuit() {
    use crate::status_wire::{ANYONE, ANYONE_WAIT};
    let s = Status { stage: ANYONE_WAIT, route: ANYONE, total: GB, done: 0, rate: 0, try_n: 3, tries: 5 };
    assert_eq!(Status::decode(&s.encode()), Some(s));
    assert_eq!(progress_line(&s), "Anyone is building its circuit, step 3 of 5");
    let starting = Status { try_n: 0, ..s };
    assert_eq!(progress_line(&starting), "Anyone is starting; the download waits for it");
}

/* A fetcher with nothing to count yet says it is starting; the store reads
 * that, where it once had no answer at all and timed out. */
#[test]
fn a_fetcher_starting_answers_and_the_card_says_so() {
    use crate::status_wire::STARTING;
    let s = Status { stage: STARTING, route: ANYONE, total: 0, done: 0, rate: 0, try_n: 0, tries: 0 };
    assert_eq!(Status::decode(&s.encode()), Some(s), "a store reads the starting stage");
    assert!(progress_line(&s).starts_with("The download is starting"));
    let mut unknown = s.encode();
    unknown[4] = STARTING + 1;
    assert_eq!(Status::decode(&unknown), None, "a stage past the last is still refused");
}

/* An install waiting behind another says which, and how far it has got. */
#[test]
fn a_queued_install_says_what_it_waits_behind() {
    use crate::store::progress_text::waiting_behind;
    let s = Status { stage: FETCHING, route: ANYONE, total: 2 * GB, done: GB / 16, rate: 0, try_n: 0, tries: 0 };
    let how = progress_line(&s);
    let said = String::from_utf8(waiting_behind(b"Qwen (large, 3B)", Some(&how))).unwrap();
    assert!(said.starts_with("Waiting: Qwen (large, 3B) installs first ("), "{said}");
    assert!(said.contains(&how) && said.ends_with("This one starts when it ends."), "{said}");
    let bare = String::from_utf8(waiting_behind(b"Ripgrep", None)).unwrap();
    assert_eq!(bare, "Waiting: Ripgrep installs first. This one starts when it ends.");
}
