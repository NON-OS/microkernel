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

//! One bring-up attempt against a modelled part: it comes up under a drawn
//! address, or it gives back every grant and the claim before it says no.

use nonos_libc::{attach, entropy, given_back};

use crate::constants::regs::REG_MAC0;
use crate::constants::MAC_LEN;
use crate::init::finish;
use crate::part::{driver_over, part, Refused, DEVICE, PIO_GRANT, RX_GRANT, TX_GRANT};
use crate::setup::Driver;

/// The grants `driver()` records, released in reverse setup order and then
/// the claim, which on its own tears the rest down as well.
const EVERYTHING: [(&str, u64); 4] = [
    ("dma_unmap", TX_GRANT),
    ("dma_unmap", RX_GRANT),
    ("pio_release", PIO_GRANT),
    ("device_release", DEVICE),
];

fn driver() -> Driver {
    driver_over(0)
}

fn released_since(before: usize) -> Vec<(&'static str, u64)> {
    given_back()[before..].to_vec()
}

#[test]
fn a_part_that_never_leaves_reset_gives_back_every_grant_and_the_claim() {
    let _part = part(true);
    let before = given_back().len();
    assert_eq!(finish(driver()).err(), Some("rtl8139 reset timeout"));
    assert_eq!(released_since(before), EVERYTHING);
}

#[test]
fn a_part_that_cannot_get_an_address_gives_back_every_grant_and_the_claim() {
    let _turn = entropy(false);
    let _part = part(false);
    let before = given_back().len();
    assert_eq!(finish(driver()).err(), Some("rtl8139 no entropy for station address"));
    assert_eq!(released_since(before), EVERYTHING);
}

#[test]
fn a_port_the_broker_refuses_gives_back_every_grant_and_the_claim() {
    let _port = attach(Box::new(Refused));
    let before = given_back().len();
    assert_eq!(finish(driver()).err(), Some("pio write failed"));
    assert_eq!(released_since(before), EVERYTHING);
}

#[test]
fn a_part_that_comes_up_keeps_every_grant_and_filters_on_the_drawn_address() {
    let _turn = entropy(true);
    let _part = part(false);
    let before = given_back().len();
    let d = finish(driver()).expect("brought up");
    assert!(released_since(before).is_empty(), "a working attempt releases nothing");
    let mut idr = [0u8; MAC_LEN];
    for (i, b) in idr.iter_mut().enumerate() {
        *b = d.pio.r8(REG_MAC0 + i as u16).expect("idr reads back");
    }
    assert_eq!(idr, d.mac);
}
