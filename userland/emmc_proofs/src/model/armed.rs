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

//! The host as a driver that died without its Drop left it: ADMA2 still
//! running from that driver's descriptor table, a page the kernel freed at
//! the release. It sits between the capsule's register window and the Sim
//! (`armed_bus`), and leaves the Sim as it is. Until a Software Reset For
//! All, or For DAT Line, is seen to finish (its bit read back clear), Present
//! State shows the engine `busy` (Command Inhibit (DAT) and DAT Line Active,
//! unless a test says otherwise), and the engine runs: on a host that
//! `runs_through_reset`, after it too.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_libc::Engines;

use super::armed_bus::plug;
use super::card::CardCfg;
use super::host::{HostCfg, Sim, SimBus};

/// Present State's Command Inhibit (DAT) and DAT Line Active.
const DAT_BUSY: u32 = 1 << 1 | 1 << 2;

pub struct Armed {
    pub sim: SimBus,
    /// Where the left-over engine fetches from; none once it is stopped.
    pub from: Option<u64>,
    /// Software Reset bits written that end DMA, not yet read back clear.
    pub(super) pending: u8,
    /// Software Resets For All seen to finish: the stop of every engine.
    pub resets: u32,
    /// The reset bit clears while ADMA still runs, as on a broken host.
    pub runs_through_reset: bool,
    /// The Present State bits the engine shows while it runs.
    pub busy: u32,
}

impl Armed {
    /// A host on this thread's broker clock whose ADMA2 engine a dead driver
    /// left running from `from` (the firmware setting the Sim starts with
    /// selects ADMA2): plugged in for the capsule's register window, and read
    /// by the broker shim at every call.
    pub fn left_running(host: HostCfg, card: CardCfg, from: u64) -> Rc<RefCell<Self>> {
        let sim = SimBus(Rc::new(RefCell::new(Sim::new(host, card, nonos_libc::uptime()))));
        sim.0.borrow_mut().write(0x58, 4, from as u32);
        sim.0.borrow_mut().write(0x5c, 4, (from >> 32) as u32);
        let (pending, resets, runs_through_reset, busy) = (0, 0, false, DAT_BUSY);
        let armed = Self { sim, from: Some(from), pending, resets, runs_through_reset, busy };
        let armed = Rc::new(RefCell::new(armed));
        plug(armed.clone());
        let watched = armed.clone();
        nonos_libc::watch(move || watched.borrow().engines());
        armed
    }

    /// The host as the broker shim's watch reads it.
    pub fn engines(&self) -> Engines {
        let running = if self.from.is_some() { self.busy } else { 0 };
        Engines { resets: self.resets, running, ports: [0; 4] }
    }
}
