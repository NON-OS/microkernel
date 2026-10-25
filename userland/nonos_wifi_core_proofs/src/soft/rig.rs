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
    /// A second AP, for roaming; heard only on its own channel.
    pub ap2: Option<SimAp>,
    pub(crate) beacon_at2: u64,
    /// The signal each AP is heard at.
    pub rssi: [i8; 2],
    /// The address the station sends from now, and whether to refuse a new one.
    pub addr: ([u8; 6], bool),
    /// The address the APs still send to, when a reset they never saw changed it.
    pub chat_to: Option<[u8; 6]>,
}

/// A ready station in range of `ap`.
pub fn rig(ap: SimAp) -> Station {
    let air = crate::soft::air_new::air(ap);
    let clock = air.clock.clone();
    SoftStation::new(Some(air), SimEnv { clock, draws: 0 }, 0)
}
