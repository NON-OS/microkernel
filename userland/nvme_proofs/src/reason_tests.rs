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

/// The failure listed after `e`, through a match the compiler holds
/// exhaustive: a new variant fails to build here until it is listed.
fn after(e: NvmeError) -> Option<NvmeError> {
    use NvmeError::*;
    match e {
        ClaimFailed => Some(BrokerCallFailed),
        BrokerCallFailed => Some(UnsupportedController),
        UnsupportedController => Some(UnsupportedPageSize),
        UnsupportedPageSize => Some(NoNvmCommandSet),
        NoNvmCommandSet => Some(ControllerTimeout),
        ControllerTimeout => Some(ControllerFatal),
        ControllerFatal => Some(ClockFailed),
        ClockFailed => Some(AdminCommandFailed),
        AdminCommandFailed => Some(InvalidTransfer),
        InvalidTransfer => Some(ShutdownTimeout),
        ShutdownTimeout => Some(DmaGrantShort),
        DmaGrantShort => Some(QueueTooShallow),
        QueueTooShallow => None,
    }
}

/// Every failure, each given its words.
fn every_failure() -> Vec<NvmeError> {
    core::iter::successors(Some(NvmeError::ClaimFailed), |&e| after(e)).collect()
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
