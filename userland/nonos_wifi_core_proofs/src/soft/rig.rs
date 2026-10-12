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

//! The simulated air and a station built on it, as a chip driver builds one.

use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::soft::ap::SimAp;
use crate::soft::env::SimEnv;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::{Association, SoftStation};

pub type Station = SoftStation<SimAir, SimEnv>;

pub struct SimAir {
    pub ap: SimAp,
    pub clock: Rc<Cell<u64>>,
    pub tuned: Option<(Band, u8)>,
    pub bss: Option<[u8; 6]>,
    pub sent: Vec<Vec<u8>>,
    /// The channel each sent frame went out on.
    pub sent_on: Vec<Option<(Band, u8)>>,
    pub inbox: VecDeque<Vec<u8>>,
    pub(crate) beacon_at: u64,
    /// Every association the station reported, in order.
    pub assoc: Vec<Option<Association>>,
    /// The radio says it runs GCMP-256 as well as CCMP-128.
    pub gcmp: bool,
}

impl SimAir {
    pub fn on_channel(&self) -> bool {
        self.ap.on && self.tuned == Some((Band::of_legacy(self.ap.channel), self.ap.channel))
    }
}

/// A ready station in range of `ap`.
pub fn rig(ap: SimAp) -> Station {
    let clock = Rc::new(Cell::new(0));
    let (sent, inbox) = (Vec::new(), VecDeque::new());
    let air = SimAir {
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
    };
    SoftStation::new(Some(air), SimEnv { clock, draws: 0 }, 0)
}
