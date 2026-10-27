// NONOS Operating System (AGPL-3.0-or-later)
//! The driver over the modelled chip: the rings laid out as `map_rings`
//! lays them, in host memory the card reaches, then the capsule's own
//! bring-up (`bring::up`) with the capsule's own address draw.

use super::bus::{self, Mem};
use super::card::Card;
use super::state::State;
use crate::bring::up::{up, Failed};
use crate::ring::addrs::{ACH0, CH12, CH8, RPQ, RXQ};
use crate::ring::rx::{self, RxRing};
use crate::ring::set::*;
use crate::ring::tx::{self, TxRing};
use crate::ring::Rings;
use crate::sta::chip::Chip;

pub type Up = Chip<Card, Mem>;

/// Every ring, each in its own region.
pub fn rings(card: &Card) -> Rings<Mem> {
    let mut s = card.0.borrow_mut();
    let mut map = |n| bus::alloc(&mut s.bus, n);
    let ach0 = TxRing::new(map(tx::bytes_for(ACH0_LEN)), ACH0, ACH0_LEN).unwrap();
    let ch8 = TxRing::new(map(tx::bytes_for(CH8_LEN)), CH8, CH8_LEN).unwrap();
    let ch12 = TxRing::new(map(tx::bytes_for(CH12_LEN)), CH12, CH12_LEN).unwrap();
    let idle = map(4096);
    let rxq = RxRing::new(map(rx::bytes_for(RXQ_LEN, RXQ_BUF)), RXQ, RXQ_LEN, RXQ_BUF).unwrap();
    let rpq = RxRing::new(map(rx::bytes_for(RPQ_LEN, RPQ_BUF)), RPQ, RPQ_LEN, RPQ_BUF).unwrap();
    Rings { ach0, ch8, ch12, idle, rxq, rpq }
}

/// A fresh chip, shaped by `setup` before the driver touches it, and the
/// driver brought up on it.
pub fn bring(setup: impl FnOnce(&mut State)) -> (Card, Result<Up, Failed<Card>>) {
    let card = Card::new();
    setup(&mut card.0.borrow_mut());
    let rings = rings(&card);
    let got = up(card.clone(), rings, crate::station::draw);
    (card, got)
}

/// A chip brought up with nothing wrong: the driver, or the proof fails.
pub fn ready() -> (Card, Up) {
    let (card, got) = bring(|_| {});
    match got {
        Ok(chip) => (card, chip),
        Err((_, stage, why)) => {
            let v = card.0.borrow().violations.clone();
            panic!("stopped at {stage:?}: {} {v:?}", String::from_utf8_lossy(why))
        }
    }
}
