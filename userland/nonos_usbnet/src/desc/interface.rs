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

//! The interfaces of a configuration, each alternate setting on its own,
//! with the bulk pipes it carries.

use alloc::vec::Vec;

use super::endpoint::endpoint;
use super::kinds::{ENDPOINT, INTERFACE, SS_ENDPOINT_COMPANION};
use super::walk::walk;
use crate::bus::Pipes;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Interface {
    pub number: u8,
    pub alt: u8,
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
    /// Its bulk pipes; an address of 0 is a direction it lacks.
    pub pipes: Pipes,
}

impl Interface {
    pub fn has_bulk_pair(&self) -> bool {
        self.pipes.bulk_in != 0 && self.pipes.bulk_out != 0
    }
}

/// Every interface descriptor in `raw`, with the first bulk IN and bulk
/// OUT endpoint that follow it and their SuperSpeed bursts.
pub fn interfaces(raw: &[u8]) -> Vec<Interface> {
    let mut out: Vec<Interface> = Vec::new();
    // The direction of the bulk endpoint a SuperSpeed companion follows;
    // `None` after any other endpoint, whose companion is not ours.
    let mut last_in: Option<bool> = None;
    for d in walk(raw) {
        match (d[1], out.last_mut()) {
            (INTERFACE, _) if d.len() >= 9 => out.push(Interface {
                number: d[2],
                alt: d[3],
                class: d[5],
                subclass: d[6],
                protocol: d[7],
                pipes: Pipes::default(),
            }),
            (ENDPOINT, Some(i)) if d.len() >= 7 => last_in = endpoint(i, d),
            (SS_ENDPOINT_COMPANION, Some(i)) if d.len() >= 3 => match last_in.take() {
                Some(true) => i.pipes.burst_in = d[2] & 0x0F,
                Some(false) => i.pipes.burst_out = d[2] & 0x0F,
                None => {}
            },
            _ => {}
        }
    }
    out
}
