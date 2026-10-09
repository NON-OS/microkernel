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

//! The DEVICE_INFO, CONTROLLER_INFO and PORT_LIST payload layouts, the
//! console line formatting, and the failure reasons.

use crate::emmc::error::{reason, EmmcError};
use crate::emmc::info::*;
use crate::emmc::mmc::Cid;
use crate::emmc::sdhci::{Caps, Snapshot};
use crate::emmc::text::Line;

#[test]
fn identify_names_the_maker_part_and_serial() {
    let cid =
        Cid { mid: 0x15, cbx: 1, oid: 0, pnm: *b"8GTF4R", prv: 0x02, psn: 0xdead_beef, mdt: 0 };
    let n = names(&cid);
    assert_eq!(n.model(), b"Samsung 8GTF4R");
    assert_eq!(&n.serial, b"DEADBEEF");
    assert_eq!(MEDIUM_EMMC, 1);
}

#[test]
fn an_unknown_maker_is_called_emmc_and_names_stay_printable() {
    let cid = Cid {
        mid: 0,
        cbx: 0,
        oid: 0,
        pnm: [b'A', 0, 0x7f, 0xff, b' ', b'z'],
        prv: 0,
        psn: 0x0000_00a1,
        mdt: 0,
    };
    let n = names(&cid);
    assert_eq!(n.model(), b"eMMC A??? z");
    assert_eq!(&n.serial, b"000000A1");
    let blank = Cid { pnm: *b"      ", ..cid };
    assert_eq!(names(&blank).model(), b"eMMC");
    for mid in 0..=255u8 {
        let n = names(&Cid { mid, pnm: [0xff; 6], ..cid });
        assert!(n.model_len <= MODEL_MAX);
        assert!(n.model().iter().all(|c| (0x20..0x7f).contains(c)));
    }
}

#[test]
fn controller_info_layout() {
    let caps = Caps { caps: 0x1462_c8b2, caps1: 0x0000_0807, version: 0x1002 };
    let snap = Snapshot {
        present: 0x1ff_0000,
        host_control: 0x36,
        power: 0x0b,
        clock: 0x0207,
        int_status: 0,
    };
    let c = controller_info(&caps, &snap);
    assert_eq!(u32::from_le_bytes(c[0..4].try_into().unwrap()), 0x1462_c8b2);
    assert_eq!(u32::from_le_bytes(c[4..8].try_into().unwrap()), 0x0207_0b36);
    assert_eq!(u32::from_le_bytes(c[8..12].try_into().unwrap()), 1);
    assert_eq!(u32::from_le_bytes(c[12..16].try_into().unwrap()), 0x1002);
    assert_eq!(u32::from_le_bytes(c[16..20].try_into().unwrap()), 0x0807);
    assert_eq!(&c[20..24], &[1, 0, 0, 0]);
}

#[test]
fn port_entry_layout() {
    let snap = Snapshot {
        present: 0x01ff_0000,
        host_control: 0,
        power: 0,
        clock: 0,
        int_status: 0x0020_8002,
    };
    let p = port_entry(true, &snap, 0xc0ff_8080, 0x900);
    assert_eq!(&p[0..4], &[0, 1, 1, PORT_KIND_EMMC]);
    assert_eq!(u32::from_le_bytes(p[4..8].try_into().unwrap()), 0x01ff_0000);
    assert_eq!(u32::from_le_bytes(p[8..12].try_into().unwrap()), 0xc0ff_8080);
    assert_eq!(u32::from_le_bytes(p[12..16].try_into().unwrap()), 0x0020_8002);
    assert_eq!(u32::from_le_bytes(p[16..20].try_into().unwrap()), 0x900);
    assert_eq!(&p[20..24], &[0; 4]);
    assert_eq!(u32::from_le_bytes(p[24..28].try_into().unwrap()), 0x20);
    assert_eq!(&p[28..36], &[0; 8]);
    assert_eq!(port_entry(false, &snap, 0, 0)[2], 0);
}

#[test]
fn console_lines() {
    let mut l = Line::new();
    l.s(b"x ")
        .hex(0)
        .s(b" ")
        .hex(0x31cc)
        .s(b" ")
        .dec(0)
        .s(b" ")
        .dec(18_446_744_073_709_551_615)
        .s(b" ")
        .ascii(b"ab\x01c\0d");
    assert_eq!(l.as_bytes(), b"driver.emmc: x 0x0 0x31cc 0 18446744073709551615 ab?c");
    let mut long = Line::new();
    for _ in 0..100 {
        long.s(b"abc");
    }
    assert_eq!(long.as_bytes().len(), crate::emmc::text::LINE_MAX);
}

#[test]
fn every_failure_has_its_own_reason() {
    let all = [
        EmmcError::Broker(-1),
        EmmcError::Window,
        EmmcError::NotEmbedded,
        EmmcError::NoAdma,
        EmmcError::HostReset,
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
