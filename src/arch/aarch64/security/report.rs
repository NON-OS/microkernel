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

//! One boot line naming each protection and what this machine made of it,
//! so a log says whether a board ran with PAN, BTI and signed returns or
//! without them, and why, rather than leaving it to be inferred.

use crate::arch::aarch64::cpu::{has_feature, CpuFeature};
use crate::sys::serial::Line;

use super::bti::built_with_bti;
use super::pac;

fn present(feature: CpuFeature) -> &'static [u8] {
    if has_feature(feature) {
        b"on"
    } else {
        b"absent"
    }
}

fn bti() -> &'static [u8] {
    match (has_feature(CpuFeature::Bti), built_with_bti()) {
        (false, _) => b"absent",
        (true, false) => b"off, image not built with bti",
        (true, true) => b"on",
    }
}

/// UXN/PXN are in every ARMv8 stage 1 descriptor; they are named so the line
/// lists what the tables enforce as well as what the CPU was asked to.
pub fn report() {
    Line::new()
        .str(b"[HARDEN] PAN ")
        .str(present(CpuFeature::Pan))
        .str(b"; UXN/PXN on; BTI ")
        .str(bti())
        .str(b"; pac-ret ")
        .str(pac::describe())
        .str(b"; E0PD ")
        .str(present(CpuFeature::E0pd))
        .end();
}
