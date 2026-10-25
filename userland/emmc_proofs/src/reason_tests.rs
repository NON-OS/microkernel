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

//! The failure reasons and the statuses a failed request answers.

use crate::emmc::error::{reason, EmmcError};

#[test]
fn every_failure_has_its_own_reason() {
    let all = [
        EmmcError::Broker(-1),
        EmmcError::Window,
        EmmcError::NotEmbedded,
        EmmcError::NoAdma,
        EmmcError::HostReset,
        EmmcError::HostBusy,
        EmmcError::ClockUnstable,
        EmmcError::NoPower,
        EmmcError::NoVoltage,
        EmmcError::Inhibit(0),
        EmmcError::CmdTimeout(0),
        EmmcError::CmdError { cmd: 0, err: 0 },
        EmmcError::DataError { cmd: 0, err: 0 },
        EmmcError::NoCompletion(0),
        EmmcError::NoCard,
        EmmcError::CardBusy,
        EmmcError::Status { cmd: 0, status: 0 },
        EmmcError::Locked,
        EmmcError::SwitchRefused(0),
        EmmcError::BadState(0),
        EmmcError::CardStuck,
        EmmcError::NoCapacity,
        EmmcError::BusWidth,
        EmmcError::DmaAddress,
        EmmcError::OutOfRange,
        EmmcError::NoMedium,
    ];
    let mut seen = std::collections::HashSet::new();
    for e in all {
        let r = reason(e);
        assert!(r.starts_with("emmc: "), "{r}");
        assert!(seen.insert(r), "{r} twice");
        assert!(!r.contains('\u{2014}'));
    }
}

#[test]
fn a_failed_request_answers_what_the_host_or_card_said() {
    use crate::emmc::error::{wire_status, CARD_ERROR_BASE, HOST_ERROR_BASE};
    assert_eq!(wire_status(EmmcError::CmdTimeout(25)), -110);
    assert_eq!(wire_status(EmmcError::NoCompletion(18)), -110);
    assert_eq!(wire_status(EmmcError::CardStuck), -110);
    assert_eq!(
        wire_status(EmmcError::DataError { cmd: 25, err: 0x0210 }),
        -(HOST_ERROR_BASE | 25 << 16 | 0x0210)
    );
    assert_eq!(
        wire_status(EmmcError::Status { cmd: 25, status: 1 << 26 | 1 << 8 }),
        -(CARD_ERROR_BASE | 25 << 16 | 26)
    );
    assert_eq!(wire_status(EmmcError::OutOfRange), -5);
}
