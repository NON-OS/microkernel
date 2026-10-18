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

//! The thermal zone the zone proofs read: every trip point, an _AC2 out
//! of order, a _TMP of the caller's choosing, and _AL0 naming a fan.

use crate::enc::*;

pub fn zone(tmp: u64) -> Vec<u8> {
    let trip = |seg: &str, dk: u64| name_obj(seg, &int(dk));
    let body = cat(&[
        &trip("_CRT", 3732),
        &trip("_HOT", 3632),
        &trip("_PSV", 3532),
        &trip("_AC0", 3432),
        &trip("_AC1", 3332),
        &trip("_AC2", 3532),
        &trip("_TZP", 300),
        &method("_TMP", 0, &ret(&int(tmp))),
        &name_obj("_AL0", &package(1, &[name("\\_SB.FAN0")])),
    ]);
    cat(&[
        &thermal_zone("\\_TZ.TZ0", &body),
        &device("\\_SB.FAN0", &name_obj("_HID", &int(0x0B0C_D041))),
    ])
}
