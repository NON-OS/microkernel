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

//! The write-protect switch, which only the host reads (sdmmc_get_ro):
//! honoured before any write command goes to the card. A reader gone from
//! the bus has no switch to read: the write is an I/O error, not a
//! write-protected card, and the card is checked before the next request.

use super::rig::{knobs, sdhc, serving};
use super::service_rig::{call, span, KERNEL};
use crate::model::{with, Knobs};
use crate::protocol::*;
use crate::serve::medium::Medium;

#[test]
fn the_write_protect_switch_refuses_a_write_and_the_card_stays_served() {
    let mut card = sdhc(1);
    card.wp = true;
    let (mut drv, mut medium) = serving(card);
    let w = call(&mut drv, &mut medium, KERNEL, OP_WRITE, &[span(5, 1), vec![0xAB; 512]].concat());
    assert_eq!(w.status, E_ROFS);
    with(|r| assert!(!r.sent.iter().any(|&c| c == 24 || c == 25), "{:?}", r.sent));
    with(|r| assert_eq!(r.slot.as_ref().map(|c| c.block(5)), Some(vec![0; 512])));
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(5, 1)).status, E_OK);
}

#[test]
fn a_write_to_a_reader_gone_from_the_bus_is_an_io_error_and_the_card_checked() {
    let (mut drv, mut medium) = serving(sdhc(1));
    knobs(Knobs { gone: true, ..Knobs::default() });
    let w = call(&mut drv, &mut medium, KERNEL, OP_WRITE, &[span(5, 1), vec![7; 512]].concat());
    knobs(Knobs::default());
    assert_eq!(w.status, E_IO);
    assert!(matches!(medium, Medium::Check(_)), "{medium:?}");
}
