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
//! The Cirrus Logic init sequences `cirrus` sends, as cs420x.c lists them
//! (sound/hda/codecs/cirrus/cs420x.c at 21a7f44fd076): CS4206 and CS4207's
//! errata sequence (:214-235) then their coefficients (:167-184), and
//! CS4208's coefficients (:186-193). Values only, no code taken.

use crate::constants::{VERB_SET_COEF_INDEX, VERB_SET_POWER_STATE, VERB_SET_PROC_COEF};

/// SET_PROCESSING_STATE (Intel HD Audio 1.0a, 7.3.3.4).
pub const SET_PROC_STATE: u16 = 0x703;
const POWER: u16 = VERB_SET_POWER_STATE;
const IDX: u16 = VERB_SET_COEF_INDEX as u16;
const COEF: u16 = VERB_SET_PROC_COEF as u16;

/// One verb: widget, verb, payload.
pub type Verb = (u8, u16, u16);

pub const CS420X: [Verb; 24] = [
    (0x01, POWER, 0),
    (0x11, SET_PROC_STATE, 1),
    (0x11, IDX, 0x0008),
    (0x11, COEF, 0x9999),
    (0x11, IDX, 0x0017),
    (0x11, COEF, 0xa412),
    (0x11, IDX, 0x0001),
    (0x11, COEF, 0x0009),
    (0x07, POWER, 0),
    (0x08, POWER, 0),
    (0x11, IDX, 0x0017),
    (0x11, COEF, 0x2412),
    (0x11, IDX, 0x0008),
    (0x11, COEF, 0x0000),
    (0x11, IDX, 0x0001),
    (0x11, COEF, 0x0008),
    (0x11, SET_PROC_STATE, 0),
    (0x11, SET_PROC_STATE, 1),
    (0x11, IDX, 0x0003),
    (0x11, COEF, 0x146a),
    (0x11, IDX, 0x0002),
    (0x11, COEF, 0x000a),
    (0x11, IDX, 0x0004),
    (0x11, COEF, 0x0007),
];

pub const CS4208: [Verb; 6] = [
    (0x01, POWER, 0),
    (0x24, SET_PROC_STATE, 1),
    (0x24, IDX, 0x0033),
    (0x24, COEF, 0x0001),
    (0x24, IDX, 0x0034),
    (0x24, COEF, 0x1c01),
];
