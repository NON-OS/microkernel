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

//! BIPR read as rtsx_pci_isr reads it, and the slot watched while a card
//! is driven.

use crate::wire::{classify, Outcome};

#[test]
fn bipr_is_read_as_the_isr_reads_it() {
    for card in [false, true] {
        let here = 1 << 16;
        assert_eq!(classify(here, card), Outcome::Pending);
        assert_eq!(classify(here | (1 << 29), card), Outcome::Ok);
        assert_eq!(classify(here | (1 << 29) | (1 << 28), card), Outcome::Fail);
        assert_eq!(classify(here | (1 << 29) | (1 << 24), card), Outcome::Fail);
        assert_eq!(classify(u32::MAX, card), Outcome::Gone);
    }
    // No card driven: an empty slot or a card-detect change is no failure.
    assert_eq!(classify(0, false), Outcome::Pending);
    assert_eq!(classify((1 << 25) | (1 << 29), false), Outcome::Ok);
}

#[test]
fn a_driven_card_gone_or_changed_wins_over_every_completion() {
    // SD_EXIST clear, or SD_INT latched, ends the run as TRANS_NO_DEVICE.
    for done in [0, 1 << 29, (1 << 29) | (1 << 28)] {
        assert_eq!(classify(done, true), Outcome::Removed);
        assert_eq!(classify(done | (1 << 16) | (1 << 25), true), Outcome::Removed);
    }
}
