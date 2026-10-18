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

//! The give-up line names the last failure in words; those words replaced
//! the per-error exit codes.

use crate::error::{reason, BgaError};

/// Every failure, so a new one fails to build here until it is given words.
const ALL: [BgaError; 6] = [
    BgaError::BrokerCallFailed(0),
    BgaError::RegisterWindowTooSmall,
    BgaError::NotDispi,
    BgaError::ModeTooLarge,
    BgaError::VideoMemoryTooSmall,
    BgaError::ModeNotTaken,
];

#[test]
fn a_refusal_names_the_driver_whatever_errno_the_broker_gave() {
    let w = reason(BgaError::BrokerCallFailed(-16));
    assert!(w.starts_with("bga: ") && w.len() > "bga: ".len(), "{w:?}");
    assert!(w.len() <= 80, "{w:?} is too long for the give-up line");
    assert_eq!(w, reason(BgaError::BrokerCallFailed(-1)));
    match BgaError::BrokerCallFailed(0) {
        BgaError::BrokerCallFailed(_)
        | BgaError::RegisterWindowTooSmall
        | BgaError::NotDispi
        | BgaError::ModeTooLarge
        | BgaError::VideoMemoryTooSmall
        | BgaError::ModeNotTaken => {}
    }
}

#[test]
fn every_refusal_says_so_in_words_of_its_own() {
    for (i, e) in ALL.iter().enumerate() {
        let w = reason(*e);
        assert!(w.starts_with("bga: ") && w.len() <= 80, "{w:?}");
        assert!(ALL[..i].iter().all(|o| reason(*o) != w), "{e:?} shares its words");
    }
}
