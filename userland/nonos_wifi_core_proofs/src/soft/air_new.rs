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

//! A fresh simulated air around one AP.

use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::soft::ap::SimAp;
use crate::soft::rig::SimAir;

/// The air with `ap` in it, its clock at 0.
pub fn air(ap: SimAp) -> SimAir {
    let clock = Rc::new(Cell::new(0));
    let (sent, inbox) = (Vec::new(), VecDeque::new());
    SimAir {
        ap,
        clock: clock.clone(),
        tuned: None,
        bss: None,
        sent,
        sent_on: Vec::new(),
        inbox,
        beacon_at: 0,
        assoc: Vec::new(),
        gcmp: false,
        ap2: None,
        beacon_at2: 0,
        rssi: [-50, -50],
        addr: (crate::soft::air::STA, false),
        chat_to: None,
    }
}
