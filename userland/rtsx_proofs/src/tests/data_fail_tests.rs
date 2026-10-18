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

//! A transfer the SD engine ends in an error. The chip raises TRANS_FAIL
//! for it: CMD12 still ends a multi-block transfer, a write is waited on
//! until the card has programmed what it took, the request answers E_IO
//! with nothing claimed, and the card is identified again before the next.
//! Should the chip raise TRANS_OK all the same, SD_TRANSFER read back
//! after the check fails the transfer.

use super::card_rw_tests::{ready, sent};
use super::rig::{knobs, sdhc, serving};
use super::service_rig::{call, span, KERNEL};
use crate::card::{read_blocks, write_blocks};
use crate::error::RtsxError;
use crate::model::{with, Knobs};
use crate::protocol::*;
use crate::serve::medium::Medium;

fn data_error(quiet_error: bool) {
    knobs(Knobs { data_error: true, quiet_error, ..Knobs::default() });
    with(|r| r.sent.clear());
}

#[test]
fn a_read_that_ends_in_an_error_answers_io_and_has_the_card_checked() {
    let (mut drv, mut medium) = serving(sdhc(1));
    with(|r| r.slot.as_mut().map(|c| c.data.insert(2, vec![0x5A; 512])));
    data_error(false);
    let r = call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 8));
    assert_eq!((r.status, r.payload.len()), (E_IO, 0));
    assert_eq!(sent(), [18, 12]);
    assert!(matches!(medium, Medium::Check(_)), "{medium:?}");
    assert!((0..8 * 512).all(|i| drv.data.read_u8(i) == 0), "data left in the buffer");
}

#[test]
fn a_write_that_ends_in_an_error_waits_out_programming_and_claims_nothing() {
    let (mut drv, mut medium) = serving(sdhc(1));
    data_error(false);
    let w = call(&mut drv, &mut medium, KERNEL, OP_WRITE, &[span(3, 8), vec![9; 4096]].concat());
    assert_eq!((w.status, w.payload.len()), (E_IO, 0));
    assert_eq!(sent(), [25, 12, 13, 13, 13], "CMD12, then CMD13 until programmed");
    assert!(matches!(medium, Medium::Check(_)), "{medium:?}");
}

#[test]
fn an_error_the_chip_raises_no_trans_fail_for_is_read_back_and_fails() {
    let (drv, card) = ready(sdhc(1));
    data_error(true);
    assert_eq!(read_blocks(&drv, &card, 0, 1), Err(RtsxError::DataFailed));
    with(|r| assert!(r.cmd_stopped && r.dma_stopped, "engines left armed"));
    assert_eq!(write_blocks(&drv, &card, 0, 1), Err(RtsxError::DataFailed));
    assert_eq!(sent(), [17, 24, 13, 13, 13], "the block it took, waited out");
}
