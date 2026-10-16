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

//! Card reads and writes at the command level: CMD12 after a multi-block
//! transfer and not after a single one, a write waited on with CMD13 until
//! programmed, an error in that status failing the write, and a card
//! pulled mid-write.

use nonos_libc::now_ms;

use super::rig::{knobs, sdhc, up};
use crate::card::{bring_up, read_blocks, write_blocks, Card};
use crate::error::RtsxError;
use crate::model::{with, Knobs, WP_VIOLATION};
use crate::setup::Driver;

pub fn ready(card: crate::model::Card) -> (Driver, Card) {
    let mut drv = up(Some(card));
    let card = bring_up(&mut drv).expect("the card comes up");
    with(|r| r.sent.clear());
    (drv, card)
}

pub fn sent() -> Vec<u8> {
    with(|r| std::mem::take(&mut r.sent))
}

#[test]
fn cmd12_follows_a_multi_block_transfer_and_cmd13_waits_out_programming() {
    let (drv, card) = ready(sdhc(1));
    assert_eq!(read_blocks(&drv, &card, 0, 1).map(|_| sent()), Ok(vec![17]));
    assert_eq!(read_blocks(&drv, &card, 0, 9).map(|_| sent()), Ok(vec![18, 12]));
    assert_eq!(write_blocks(&drv, &card, 0, 1).map(|_| sent()), Ok(vec![24, 13, 13, 13]));
    assert_eq!(write_blocks(&drv, &card, 0, 9).map(|_| sent()), Ok(vec![25, 12, 13, 13, 13]));
}

#[test]
fn an_error_the_card_reports_after_a_write_fails_it() {
    let (drv, card) = ready(sdhc(1));
    with(|r| r.slot.as_mut().map(|c| c.errors = WP_VIOLATION));
    assert_eq!(write_blocks(&drv, &card, 0, 1), Err(RtsxError::CardError));
}

#[test]
fn a_card_pulled_mid_write_ends_it_at_once_with_no_cmd12() {
    let (drv, card) = ready(sdhc(1));
    knobs(Knobs { pull_on_data: true, ..Knobs::default() });
    let t0 = now_ms();
    assert_eq!(write_blocks(&drv, &card, 0, 8), Err(RtsxError::Removed));
    assert!(now_ms() - t0 < 50, "{} ms", now_ms() - t0);
    assert_eq!(sent(), [25]);
    with(|r| assert!(r.cmd_stopped && r.dma_stopped));
}
