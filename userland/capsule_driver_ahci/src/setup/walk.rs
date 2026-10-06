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

//! The walk over every controller discovery found and every port on it.

use alloc::vec::Vec;

use super::open::open;
use super::probe::probe;
use super::say_port::say_port;
use super::say_skipped::say_skipped;
use super::walked::{Probed, Walk};
use crate::choose::Candidate;
use crate::discover::Found;
use crate::engine::{init_port, may_be_disk};
use crate::error::{reason, AhciError};
use crate::log::Line;

/// Open every controller in `found` and try every implemented port on each
/// whose signature names no other device kind. The port's own COMRESET
/// decides whether a disk is there: after the staggered spin-up a port's
/// link state read before it is no guide. init_port keeps only an ATA disk.
/// The probe sectors of each disk are read. `Candidate::controller` is the
/// controller's index in `opened`.
pub(super) fn walk(found: &[Found]) -> Walk {
    let mut w =
        Walk { opened: Vec::new(), probed: Vec::new(), first_error: None, first_port_error: None };
    for dev in found {
        let ctl = match open(*dev) {
            Ok(ctl) => ctl,
            Err(e) => {
                Line::new().text(b"controller not opened: ").text(reason(e).as_bytes()).send();
                w.first_error.get_or_insert(e);
                continue;
            }
        };
        let controller = w.opened.len();
        for p in ctl.ports.iter().filter(|p| p.implemented == 1) {
            if !may_be_disk(p.sig) {
                say_skipped(p.index, p.sig);
                continue;
            }
            let up = init_port(dev.device_id, ctl.epoch, ctl.regs, ctl.info.cap, p.index);
            say_port(ctl.regs, p.index, &up);
            let mut port = match up {
                Ok(port) => port,
                Err(AhciError::NoDisk) => continue,
                Err(e) => {
                    w.first_port_error.get_or_insert(e);
                    continue;
                }
            };
            let (store, plan) = probe(&mut port, ctl.regs);
            let cand = Candidate { controller, port: p.index, store, plan };
            w.probed.push(Probed { port, cand });
        }
        w.opened.push(ctl);
    }
    w
}
