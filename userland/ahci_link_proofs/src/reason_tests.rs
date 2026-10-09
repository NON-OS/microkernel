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
//! the per-error exit codes, so every cause has to read apart from the rest.

use crate::error::{reason, AhciError};
use crate::identity::Refusal;

/// Every failure, listed through a match the compiler holds exhaustive: a new
/// variant fails to build here until it is listed and given its words.
fn every_failure() -> Vec<AhciError> {
    let all = [
        AhciError::DeviceNotFound,
        AhciError::NoDisk,
        AhciError::BrokerCallFailed(-16),
        AhciError::CommandFailed,
        AhciError::Timeout,
        AhciError::IdentityRefused(Refusal::NoCapacity),
        AhciError::OutOfRange,
    ];
    for e in all {
        match e {
            AhciError::DeviceNotFound
            | AhciError::NoDisk
            | AhciError::BrokerCallFailed(_)
            | AhciError::CommandFailed
            | AhciError::Timeout
            | AhciError::IdentityRefused(_)
            | AhciError::OutOfRange => {}
        }
    }
    all.to_vec()
}

#[test]
fn every_failure_has_its_own_words_naming_the_driver() {
    let words: Vec<&str> = every_failure().into_iter().map(reason).collect();
    for (i, w) in words.iter().enumerate() {
        assert!(w.starts_with("ahci: ") && w.len() > "ahci: ".len(), "{w:?}");
        assert!(!words[i + 1..].contains(w), "{w:?} names two failures");
    }
}

#[test]
fn a_refusal_reads_the_same_whatever_errno_the_broker_gave() {
    assert_eq!(reason(AhciError::BrokerCallFailed(-16)), reason(AhciError::BrokerCallFailed(-1)));
}

#[test]
fn the_words_fit_the_give_up_line() {
    for e in every_failure() {
        assert!(reason(e).len() <= 80, "{:?} is too long for the line", reason(e));
    }
}
