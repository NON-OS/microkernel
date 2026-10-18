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

//! The served card pulled while it is identified again, wherever in
//! identification the pull lands, is away and not another card: the
//! slot answers no medium until it is back, and then it is served again.

use super::medium_tests::read;
use super::rig::{knobs, sdhc, serving};
use crate::model::{with, Knobs, SD_INT};
use crate::protocol::*;
use crate::serve::medium::Medium;

/// The served card in doubt, pulled as identification sends CMD`index`,
/// then put back.
fn pulled_while_identified_again(index: u8) {
    let (mut drv, mut medium) = serving(sdhc(1));
    knobs(Knobs { pull_on_cmd: Some(index), ..Knobs::default() });
    with(|r| r.pending |= SD_INT);
    assert_eq!(read(&mut drv, &mut medium), E_NOMEDIUM, "pulled at CMD{index}");
    assert!(matches!(medium, Medium::Away(_)), "{medium:?}");
    with(|r| r.insert(sdhc(1)));
    assert_eq!(read(&mut drv, &mut medium), E_OK, "the same card back");
    assert!(matches!(medium, Medium::Ready(_)), "{medium:?}");
}

#[test]
fn the_served_card_pulled_while_identified_again_is_served_once_it_is_back() {
    pulled_while_identified_again(41);
}

/// CMD8 failing is no failure, so a pull there is first met at the CMD55
/// before ACMD41, inside the ACMD41 loop.
#[test]
fn a_pull_at_cmd8_is_away_too() {
    pulled_while_identified_again(8);
}

#[test]
fn a_pull_at_the_cmd55_before_acmd41_is_away_too() {
    pulled_while_identified_again(55);
}
