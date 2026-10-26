// NONOS Operating System (AGPL-3.0-or-later)
//! Told of an association, power save reports the station connected, lays
//! the pages down at the driver's first reserved page, names them to the
//! firmware and sets the AID and DTIM period; only then may it sleep.

use super::join_card::{JoinCard, Mem};
use crate::h2c::cmds::{media_status, rsvd_page};
use crate::pcie::Aspm;
use crate::ps::pages::{sta_pages, NULL_PAGE, PS_POLL_PAGE, QOS_NULL_PAGE};
use crate::ps::{Bss, Ps, Seen, PERIOD_MS};

const BSS: Bss =
    Bss { aid: 5, dtim: 3, bssid: [0x02, 0xAA, 0, 0, 0, 1], sta: [0x02, 0x11, 0, 0, 0, 2] };
const BOX0: usize = 0x01D0;
const BOX1: usize = 0x01D4;

fn told(lands: bool, beacon: bool) -> (Ps, JoinCard, Mem) {
    let (card, stage, ring) = (JoinCard::new(lands), Mem::default(), Mem::default());
    let mut ps = Ps::new(Aspm::default());
    ps.tell_joined(&card, beacon.then_some((&stage, &ring)), BSS);
    let quiet = Seen { tx: 0, rx: 0, sends: 0 };
    ps.step(&card, 0, quiet, true);
    ps.step(&card, PERIOD_MS, quiet, true);
    (ps, card, stage)
}

#[test]
fn an_association_is_told_in_rtw88s_order_and_the_station_may_sleep() {
    let (ps, card, stage) = told(true, true);
    assert!(ps.asleep());
    let media = card.at(BOX0)[0];
    assert_eq!(media.1, media_status(true, 0).0, "the station connected, first");
    let heads = card.at(0x0204);
    assert!(heads.iter().all(|&(i, v)| v == 0x8000 | 460 && i > media.0), "at page 460");
    let (w0, _) = rsvd_page(PS_POLL_PAGE, NULL_PAGE, QOS_NULL_PAGE);
    assert!(card.at(BOX1)[0].1 == w0 && card.at(BOX1)[0].0 > heads[0].0, "then where they are");
    assert_eq!(card.at(0x06A8), [(card.at(0x06A8)[0].0, 0xF000_0005)], "AID, the rest kept");
    assert_eq!(card.at(0x05A8)[0].1, 2, "DTIM period less one");
    let pages = sta_pages(BSS.aid, BSS.bssid, BSS.sta);
    let s = stage.0.borrow();
    assert_eq!(&s[49..48 + pages.len()], &pages[1..], "the pages behind the download's descriptor");
    assert_eq!((s[0], s[48]), (1, pages[0]), "then the stand-in alone, over page 0");
}

#[test]
fn pages_that_never_land_or_no_beacon_ring_keep_the_station_awake() {
    assert!(!told(false, true).0.asleep(), "the pages never landed");
    let (ps, card, _) = told(true, false);
    assert!(!ps.asleep(), "nothing to lay them down through");
    assert_eq!(card.at(0x05A8)[0].1, 2, "the registers are set regardless");
}

#[test]
fn leaving_reports_the_station_gone_only_where_it_was_reported_connected() {
    let (mut ps, card, _) = told(true, true);
    ps.left(&card);
    assert!(!ps.asleep());
    assert_eq!(card.at(BOX0).last().map(|w| w.1), Some(media_status(false, 0).0), "gone");
    let (mut awake, card, _) = told(false, true);
    let before = card.writes.borrow().len();
    awake.left(&card);
    assert_eq!(card.writes.borrow().len(), before, "never connected, nothing to say");
}
