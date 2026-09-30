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

use alloc::vec::Vec;

use super::open::{open, Opened};
use super::probe::probe;
use crate::choose::Candidate;
use crate::constants::PORT_KIND_SATA;
use crate::discover::Found;
use crate::engine::{init_port, Port};
use crate::error::AhciError;

/// A port that came up, kept running until the choice is made.
pub(super) struct Probed {
    pub port: Port,
    pub cand: Candidate,
}

/// Everything the walk opened and brought up.
pub(super) struct Walk {
    pub opened: Vec<Opened>,
    pub probed: Vec<Probed>,
    /// The first controller that could not be opened, if any.
    pub first_error: Option<AhciError>,
}

/// Open every controller in `found` and bring up every present SATA port on
/// each, reading the probe sectors of each disk. `Candidate::controller` is
/// the controller's index in `opened`.
pub(super) fn walk(found: &[Found]) -> Walk {
    let mut w = Walk { opened: Vec::new(), probed: Vec::new(), first_error: None };
    for dev in found {
        let ctl = match open(*dev) {
            Ok(ctl) => ctl,
            Err(e) => {
                w.first_error.get_or_insert(e);
                continue;
            }
        };
        let controller = w.opened.len();
        for p in ctl.ports.iter().filter(|p| p.present == 1 && p.kind == PORT_KIND_SATA) {
            let Ok(mut port) = init_port(dev.device_id, ctl.epoch, ctl.regs, p.index) else {
                continue;
            };
            let (store, plan) = probe(&mut port, ctl.regs);
            let cand = Candidate { controller, port: p.index, store, plan };
            w.probed.push(Probed { port, cand });
        }
        w.opened.push(ctl);
    }
    w
}
