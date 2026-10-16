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

//! A card is told from another by its CID, not its size: the served card
//! out and back is served again, and any other card in its place ends
//! service for good, even if the first comes back after it.

use super::rig::{sdhc, serving, up};
use super::service_rig::{call, span, KERNEL};
use crate::model::with;
use crate::protocol::*;
use crate::serve::medium::Medium;
use crate::setup::Driver;

pub fn read(drv: &mut Driver, medium: &mut Medium) -> i32 {
    call(drv, medium, KERNEL, OP_READ, &span(0, 1)).status
}

#[test]
fn an_empty_slot_answers_no_medium_until_a_card_is_put_in() {
    let mut drv = up(None);
    let mut medium = Medium::Empty;
    assert_eq!(read(&mut drv, &mut medium), E_NOMEDIUM);
    with(|r| r.insert(sdhc(1)));
    assert_eq!(read(&mut drv, &mut medium), E_OK);
}

#[test]
fn the_served_card_out_and_back_is_served_again() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let card = with(|r| r.pull()).expect("a card");
    assert_eq!(read(&mut drv, &mut medium), E_NOMEDIUM);
    assert!(matches!(medium, Medium::Away(_)), "{medium:?}");
    with(|r| r.insert(card));
    assert_eq!(read(&mut drv, &mut medium), E_OK);
}

#[test]
fn another_card_of_the_same_size_ends_service_for_good() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let first = with(|r| r.pull()).expect("a card");
    with(|r| r.insert(sdhc(2)));
    assert_eq!(read(&mut drv, &mut medium), E_NODEV);
    assert_eq!(medium, Medium::Changed);
    let w = call(&mut drv, &mut medium, KERNEL, OP_WRITE, &[span(9, 1), vec![7; 512]].concat());
    assert_eq!(w.status, E_NODEV);
    with(|r| assert_eq!(r.slot.as_ref().map(|c| c.data.len()), Some(0), "written to"));
    with(|r| r.pull().map(|_| r.insert(first)));
    assert_eq!(read(&mut drv, &mut medium), E_NODEV, "served again after a swap");
}
