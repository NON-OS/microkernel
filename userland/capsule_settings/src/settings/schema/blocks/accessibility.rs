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

use crate::settings::schema::access_knob::Knob;
use crate::settings::schema::rows::{Block, Pill, Row};

pub const ACCESSIBILITY: &[Block] = &[
    Block {
        title: "Display",
        note: Some(
            "Each display keeps its own; a mirrored one its own colours, the first's magnifier.",
        ),
        pill: Pill::None,
        rows: &[Row::Access(Knob::Output)],
    },
    Block {
        title: "Colour",
        note: None,
        pill: Pill::None,
        rows: &[Row::Access(Knob::Filter), Row::Access(Knob::Contrast)],
    },
    Block {
        title: "Magnifier",
        note: None,
        pill: Pill::None,
        rows: &[Row::Access(Knob::Magnify), Row::Access(Knob::Follow)],
    },
];
