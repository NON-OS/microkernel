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
//! the negated-errno exit codes, so every cause has to read apart.

use crate::error::{reason, XhciError};

/// Every failure, listed through a match the compiler holds exhaustive: a new
/// variant fails to build here until it is listed and given its words.
fn every_failure() -> Vec<XhciError> {
    let all = [
        XhciError::BrokerCallFailed(-16),
        XhciError::ControllerUnsupported,
        XhciError::ResetTimeout,
        XhciError::ControllerNotReadyTimeout,
        XhciError::StartTimeout,
        XhciError::HaltTimeout,
        XhciError::CommandRingFull,
        XhciError::TransferRingFull,
        XhciError::NoDeviceOnPort,
        XhciError::PortResetTimeout,
        XhciError::TransferCompletionTimeout,
        XhciError::CommandCompletionTimeout,
        XhciError::CommandCompletionFailed(4),
        XhciError::UnexpectedCompletionSlot,
        XhciError::TransferCompletionFailed(6),
    ];
    for e in all {
        match e {
            XhciError::BrokerCallFailed(_)
            | XhciError::ControllerUnsupported
            | XhciError::ResetTimeout
            | XhciError::ControllerNotReadyTimeout
            | XhciError::StartTimeout
            | XhciError::HaltTimeout
            | XhciError::CommandRingFull
            | XhciError::TransferRingFull
            | XhciError::NoDeviceOnPort
            | XhciError::PortResetTimeout
            | XhciError::TransferCompletionTimeout
            | XhciError::CommandCompletionTimeout
            | XhciError::CommandCompletionFailed(_)
            | XhciError::UnexpectedCompletionSlot
            | XhciError::TransferCompletionFailed(_) => {}
        }
    }
    all.to_vec()
}

#[test]
fn every_failure_has_its_own_words_naming_the_driver() {
    let words: Vec<&str> = every_failure().into_iter().map(reason).collect();
    for (i, w) in words.iter().enumerate() {
        assert!(w.starts_with("xhci: ") && w.len() > "xhci: ".len(), "{w:?}");
        assert!(!words[i + 1..].contains(w), "{w:?} names two failures");
    }
}

#[test]
fn the_words_fit_the_give_up_line() {
    for e in every_failure() {
        assert!(reason(e).len() <= 80, "{:?} is too long for the line", reason(e));
    }
}
