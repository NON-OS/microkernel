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
//! The clock entities of a USB Audio 2.0 AudioControl interface, and the
//! terminals that name them (2.0 4.7.2, as recalled, unverified).

use alloc::vec::Vec;

use nonos_usbnet::desc::walk;

use super::alt::{CLASS_AUDIO, CS_INTERFACE, INTERFACE, UAC2};

const SUBCLASS_CONTROL: u8 = 0x01;
const INPUT_TERMINAL: u8 = 0x02;
const OUTPUT_TERMINAL: u8 = 0x03;
const CLOCK_SOURCE: u8 = 0x0A;
const CLOCK_SELECTOR: u8 = 0x0B;
/// bmControls bits 1:0 of a Clock Source: 3, the host sets its frequency.
const PROGRAMMABLE: u8 = 0x3;

pub(super) enum Entity {
    Terminal { id: u8, clock: u8 },
    Source { id: u8, programmable: bool },
    Selector { id: u8, first: u8 },
}

/// The 2.0 AudioControl interface in `config` and its clock entities.
pub(super) fn entities(config: &[u8]) -> (u8, Vec<Entity>) {
    let (mut control, mut interface, mut seen) = (false, 0, Vec::new());
    for d in walk(config) {
        if d[1] == INTERFACE && d.len() >= 9 {
            control = d[5] == CLASS_AUDIO && d[6] == SUBCLASS_CONTROL && d[7] == UAC2;
            interface = if control { d[2] } else { interface };
            continue;
        }
        if !control || d[1] != CS_INTERFACE || d.len() < 5 {
            continue;
        }
        let id = d[3];
        seen.push(match d[2] {
            INPUT_TERMINAL if d.len() >= 17 => Entity::Terminal { id, clock: d[7] },
            OUTPUT_TERMINAL if d.len() >= 12 => Entity::Terminal { id, clock: d[8] },
            CLOCK_SOURCE if d.len() >= 8 => {
                Entity::Source { id, programmable: d[5] & 0x3 == PROGRAMMABLE }
            }
            CLOCK_SELECTOR if d.len() >= 6 && d[4] >= 1 => Entity::Selector { id, first: d[5] },
            _ => continue,
        });
    }
    (interface, seen)
}
