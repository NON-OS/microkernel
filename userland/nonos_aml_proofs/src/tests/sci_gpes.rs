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

//! The AML the SCI proofs run.

use crate::enc::*;
use crate::load::Aml;
use crate::run::boot;

/// GPE methods for 0x01 (level), 0x02 (edge), 0x0D (level, a wake event in
/// the tests that say so) and 0x50, past GPE0's 64 events.
pub fn gpes() -> Aml {
    let nop = vec![0xA3];
    let src = cat(&[
        &method("\\_GPE._L01", 0, &nop),
        &method("\\_GPE._E02", 0, &nop),
        &method("\\_GPE._L0D", 0, &nop),
        &method("\\_GPE._L50", 0, &nop),
    ]);
    boot(&src, 2).0
}
