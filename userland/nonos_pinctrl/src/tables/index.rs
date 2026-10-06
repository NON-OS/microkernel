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

use super::adln::ADL_N;
use super::adls::ADL_S;
use super::cnlh::CNL_H;
use super::cnllp::CNL_LP;
use super::icllp::ICL_LP;
use super::icln::ICL_N;
use super::jsl::JSL;
use super::mtlp::MTL_P;
use super::spth::SPT_H;
use super::sptlp::SPT_LP;
use super::tglh::TGL_H;
use super::tgllp::TGL_LP;
use crate::group::Layout;

/// Each Intel pinctrl `_HID` and the layout Linux binds it to (the
/// acpi_device_id table of each pinctrl-<platform>.c). Broxton, Apollo Lake
/// and Gemini Lake are not here: there each community is its own ACPI device
/// and the i2c driver maps them as such.
pub const LAYOUTS: &[(&[&[u8]], &Layout)] = &[
    (&[b"INT344B"], &SPT_LP),
    (&[b"INT3451", b"INT345D"], &SPT_H),
    (&[b"INT34BB"], &CNL_LP),
    (&[b"INT3450"], &CNL_H),
    (&[b"INT3455"], &ICL_LP),
    (&[b"INT34C3"], &ICL_N),
    (&[b"INT34C8"], &JSL),
    (&[b"INT34C5", b"INTC1055"], &TGL_LP),
    (&[b"INT34C6"], &TGL_H),
    (&[b"INTC1057"], &ADL_N),
    (&[b"INTC1056", b"INTC1085"], &ADL_S),
    (&[b"INTC105E", b"INTC1083"], &MTL_P),
];
