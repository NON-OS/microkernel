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

//! A Time and Alarm device as firmware writes one: _GCP, _GRT from a
//! buffer it keeps, _SRT storing what it is given, the AC timer running
//! (an hour left, policy two minutes, wake status 1) and the DC timer
//! disabled, _STV accepting the AC timer and refusing DC. A second device
//! claims the id by _CID and is absent.

use crate::enc::*;

pub fn time(year: u16, month: u8, day: u8, valid: u8) -> Vec<u8> {
    let [y0, y1] = year.to_le_bytes();
    let [z0, z1] = 60i16.to_le_bytes();
    vec![y0, y1, month, day, 13, 45, 30, valid, 250, 0, z0, z1, 1, 0, 0, 0]
}

/// If Arg0 is 0 return `ac`, else `dc`.
fn per_timer(path: &str, args: u8, ac: u64, dc: u64, extra: &[u8]) -> Vec<u8> {
    let is_ac = e(0x93, &[&arg(0), &int(0)]);
    let body = cat(&[extra, &if_(&is_ac, &ret(&int(ac))), &ret(&int(dc))]);
    method(path, args, &body)
}

pub fn machine() -> Vec<u8> {
    let now = time(2026, 10, 12, 1);
    let srt = cat(&[&store(&arg(0), &name("\\SRTB")), &ret(&int(0))]);
    let stv = store(&arg(1), &name("\\STVS"));
    let rtc = cat(&[
        &name_obj("_HID", &string("ACPI000E")),
        &name_obj("_GCP", &int(0x05)),
        &name_obj("TIME", &buffer(16, &now)),
        &method("_GRT", 0, &ret(&name("TIME"))),
        &method("_SRT", 1, &srt),
        &per_timer("_TIV", 1, 3600, 0xFFFF_FFFF, &[]),
        &per_timer("_TIP", 1, 120, 0xFFFF_FFFF, &[]),
        &per_timer("_GWS", 1, 1, 0, &[]),
        &per_timer("_STV", 2, 0, 1, &stv),
    ]);
    cat(&[
        &name_obj("\\SRTB", &buffer(1, &[0])),
        &name_obj("\\STVS", &int(0)),
        &device("\\_SB.AWAC", &rtc),
        &device(
            "\\_SB.RTC2",
            &cat(&[&name_obj("_CID", &string("ACPI000E")), &name_obj("_STA", &int(0))]),
        ),
    ])
}
