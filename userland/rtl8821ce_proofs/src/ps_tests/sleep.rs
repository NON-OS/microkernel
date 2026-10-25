// NONOS Operating System (AGPL-3.0-or-later)
//! Power save asleep and awake on a card: L1 and CLKREQ follow it where the
//! host enabled them, and nothing at all happens while the switch is off.

use super::dbi_card::DbiCard;
use crate::pcie::Aspm;
use crate::ps::{switch::LPS, Ps, Seen, PERIOD_MS};

const L1: u8 = 1 << 3;
const CLKREQ: u8 = 1 << 4;
const QUIET: Seen = Seen { tx: 0, rx: 0, sends: 0 };

fn asleep_on(card: &DbiCard, aspm: Aspm) -> Ps {
    let mut ps = Ps::new(aspm);
    ps.step(card, 0, QUIET, true);
    ps.step(card, PERIOD_MS, QUIET, true);
    assert!(ps.asleep(), "a quiet period puts it to sleep");
    ps
}

#[test]
fn asleep_the_chip_takes_l1_and_clkreq_as_far_as_the_host_allowed() {
    let card = DbiCard::default();
    card.link_cfg.set(0x01);
    let mut ps = asleep_on(&card, Aspm { l1: true, clkreq: false });
    assert_eq!(card.link_cfg.get(), 0x01 | L1, "L1 only");
    ps.wake(&card);
    assert!(!ps.asleep());
    assert_eq!(card.link_cfg.get(), 0x01, "and back");
    let both = DbiCard::default();
    asleep_on(&both, Aspm { l1: true, clkreq: true });
    assert_eq!(both.link_cfg.get(), L1 | CLKREQ);
}

#[test]
fn a_host_that_enabled_neither_sees_no_dbi_write() {
    let card = DbiCard::default();
    asleep_on(&card, Aspm::default()).wake(&card);
    assert_eq!(card.writes.get(), 0);
}

#[test]
fn a_busy_period_wakes_it_and_the_switch_off_leaves_the_card_alone() {
    let card = DbiCard::default();
    let mut ps = asleep_on(&card, Aspm { l1: true, clkreq: true });
    ps.step(&card, 2 * PERIOD_MS, Seen { tx: 999, ..QUIET }, true);
    assert!(!ps.asleep() && card.link_cfg.get() == 0);
    let off = DbiCard::default();
    let mut ps = Ps::new(Aspm { l1: true, clkreq: true });
    for t in 0..10 {
        ps.tick(&off, t * PERIOD_MS, QUIET, true);
    }
    assert!(!LPS && !ps.asleep() && off.writes.get() == 0, "the switch is off");
}
