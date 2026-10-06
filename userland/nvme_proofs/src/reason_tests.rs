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

use crate::error::{reason, NvmeError};

/// Every failure, listed through a match the compiler holds exhaustive: a new
/// variant fails to build here until it is listed and given its words.
fn every_failure() -> Vec<NvmeError> {
    let all = [
        NvmeError::ClaimFailed,
        NvmeError::BrokerCallFailed,
        NvmeError::UnsupportedController,
        NvmeError::UnsupportedPageSize,
        NvmeError::ControllerTimeout,
        NvmeError::ControllerFatal,
        NvmeError::ClockFailed,
        NvmeError::AdminCommandFailed,
        NvmeError::InvalidTransfer,
    ];
    for e in all {
        match e {
            NvmeError::ClaimFailed
            | NvmeError::BrokerCallFailed
            | NvmeError::UnsupportedController
            | NvmeError::UnsupportedPageSize
            | NvmeError::ControllerTimeout
            | NvmeError::ControllerFatal
            | NvmeError::ClockFailed
            | NvmeError::AdminCommandFailed
            | NvmeError::InvalidTransfer => {}
        }
    }
    all.to_vec()
}

#[test]
fn every_failure_has_its_own_words_naming_the_driver() {
    let words: Vec<&str> = every_failure().into_iter().map(reason).collect();
    for (i, w) in words.iter().enumerate() {
        assert!(w.starts_with("nvme: ") && w.len() > "nvme: ".len(), "{w:?}");
        assert!(!words[i + 1..].contains(w), "{w:?} names two failures");
    }
}

#[test]
fn the_words_fit_the_give_up_line() {
    // The shared give-up line holds 160 bytes; the driver name and framing
    // take about sixty of them.
    for e in every_failure() {
        assert!(reason(e).len() <= 80, "{:?} is too long for the line", reason(e));
    }
}
