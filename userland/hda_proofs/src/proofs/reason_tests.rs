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

//! What a failed bring-up says: the give-up line carries the last failure in
//! words and the setup-fail mark its number, so both must tell causes apart,
//! and a number must never read as one of the shared exit codes.

use crate::error::{exit_code, reason, HdaError};

/// Every failure, listed through a match the compiler holds exhaustive: a new
/// variant fails to build here until it is listed and given its words.
fn every_failure() -> Vec<HdaError> {
    let all = [
        HdaError::BrokerCallFailed(-16),
        HdaError::ControllerResetTimeout,
        HdaError::UnsupportedController,
        HdaError::VerbTimeout,
        HdaError::ControllerNotResponding,
        HdaError::DmaOutOfReach,
        HdaError::StreamResetTimeout,
        HdaError::CodecPowerTimeout,
    ];
    for e in all {
        match e {
            HdaError::BrokerCallFailed(_)
            | HdaError::ControllerResetTimeout
            | HdaError::UnsupportedController
            | HdaError::VerbTimeout
            | HdaError::ControllerNotResponding
            | HdaError::DmaOutOfReach
            | HdaError::StreamResetTimeout
            | HdaError::CodecPowerTimeout => {}
        }
    }
    all.to_vec()
}

#[test]
fn every_failure_has_its_own_words_naming_the_driver() {
    let words: Vec<&str> = every_failure().into_iter().map(reason).collect();
    for (i, w) in words.iter().enumerate() {
        assert!(w.starts_with("hda: ") && w.len() > "hda: ".len(), "{w:?}");
        assert!(!words[i + 1..].contains(w), "{w:?} names two failures");
        assert!(w.len() <= 80, "{w:?} is too long for the give-up line");
    }
}

#[test]
fn every_failure_keeps_its_own_mark_number() {
    let codes: Vec<i32> = every_failure().into_iter().map(exit_code).collect();
    for (i, c) in codes.iter().enumerate() {
        assert!(!codes[i + 1..].contains(c), "mark number {c} names two failures");
        // 2 is EXIT_ABSENT now; a mark must not read as "no controller".
        assert_ne!(*c, 2);
    }
}
