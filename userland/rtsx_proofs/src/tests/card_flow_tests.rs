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

//! Identification as the MMC core drives this host: the commands in order,
//! ACMD41 asked again while the card powers up and given up after its
//! second, and a standard capacity card byte addressed with a block length.

use nonos_libc::{now_ms, said};

use super::rig::{sdhc, up};
use crate::card::bring_up;
use crate::error::RtsxError;
use crate::model::{with, Card};

#[test]
fn identification_runs_in_the_order_the_mmc_core_drives_it() {
    let mut drv = up(Some(sdhc(1)));
    let card = bring_up(&mut drv).expect("the card comes up");
    // ACMD41 answers busy twice before the card is ready.
    with(|r| assert_eq!(r.sent, [0, 8, 55, 41, 55, 41, 55, 41, 2, 3, 9, 7, 55, 6]));
    assert_eq!((card.rca, card.high_capacity, card.blocks), (0x4567, true, 1 << 20));
}

#[test]
fn a_standard_capacity_card_is_byte_addressed_with_a_block_length() {
    let mut old = Card::sdsc(4095, 1);
    old.v2 = false;
    let mut drv = up(Some(old));
    let card = bring_up(&mut drv).expect("the card comes up");
    with(|r| assert_eq!(r.sent[..2], [0, 8]));
    with(|r| assert!(r.sent.ends_with(&[2, 3, 9, 7, 16, 55, 6]), "{:?}", r.sent));
    assert_eq!((card.high_capacity, card.blocks), (false, 4096 * 512));
}

#[test]
fn a_card_that_never_finishes_powering_up_is_given_up_after_a_second() {
    let mut mute = sdhc(1);
    mute.busy_polls = u32::MAX;
    let mut drv = up(Some(mute));
    let t0 = now_ms();
    assert_eq!(bring_up(&mut drv), Err(RtsxError::PowerUpTimeout));
    assert!((1_000..1_200).contains(&(now_ms() - t0)), "{} ms", now_ms() - t0);
    let named = "identification: card still busy after 1 s of ACMD41";
    assert!(said().iter().any(|l| l.contains(named)), "{:?}", said());
}
