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

//! A hub found by its configuration: brought up and kept, or its failure
//! said on the log.

use crate::hub::{attach, Line, Place};

use super::devices::Devices;
use super::types::Outcome;

pub(super) fn attach_hub(
    xhci_port: u32,
    slot: u8,
    place: Place,
    raw: &[u8],
    devs: &mut Devices,
) -> Outcome {
    match attach(xhci_port, slot, place, raw) {
        Ok(hub) => {
            devs.hubs.push(hub);
            Outcome::Bound
        }
        Err(e) => {
            let line = Line::new().text(b"root port ").num(place.root_port as u32);
            line.text(b": hub in slot ").num(slot as u32).text(b" stopped: ").text(e.says()).say();
            Outcome::Failed
        }
    }
}
