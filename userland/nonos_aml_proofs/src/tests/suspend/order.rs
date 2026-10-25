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

//! S3's timeline as ACPI 6.5 orders it (7.4, 16.1.3 and 16.1.4), on the
//! model laptop (`rig.rs`) with the policy's four devices.

pub const S3_ORDER: &[&str] = &[
    "\\_S3_()",
    "\\_S0_()",
    "\\_TTS(3)",
    "\\_SB_.XHCI._PRW()",
    "\\_SB_.PWRR._ON_()",
    "\\_SB_.XHCI._DSW(1,3,3)",
    "hw gpe 0x6d on",
    "\\_SB_.LAN0._PRW()",
    "\\_SB_.LAN0._PSW(1)",
    "hw gpe 0x6e on",
    "\\_SB_.KBD0._PRW()",
    "\\_PTS(3)",
    "\\_SI_._SST(3)",
    "hw vector 0x9a000",
    "hw clear WAK_STS",
    "hw sleep 5/5",
    "\\_SI_._SST(2)",
    "\\_WAK(3)",
    "hw gpe 0x6e off",
    "\\_SB_.LAN0._PSW(0)",
    "hw gpe 0x6d off",
    "\\_SB_.XHCI._DSW(0,0,0)",
    "\\_SB_.PWRR._OFF()",
    "\\_TTS(0)",
    "\\_SI_._SST(1)",
];
