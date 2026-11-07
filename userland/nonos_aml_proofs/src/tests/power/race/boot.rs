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

//! capsule_acpi's bring-up, step by step, in a given order, against the
//! board: the spaces connected with their _REG (boot/bringup.rs), every
//! _INI (the same), the inventory (boot/report.rs), ACPI mode entered
//! (boot/sci.rs), then the batteries and adapters read by the capsule's own
//! power/read.rs and folded by its power/word.rs; then the capsule's loop
//! (server/run.rs) every 10 ms: a stalled EC tried again (ec/retry.rs,
//! `Health::retry`) and the batteries read at once when it is, and read
//! again when power/retry.rs says a reading is owed or a minute has passed
//! (power/refresh.rs). `glue_tests` pins each to the capsule's code.

use super::super::read::{find, read_adapter, read_info, read_status};
use super::super::retry::wanted;
use super::super::state::Power;
use super::super::word::{word, BATTERY_PRESENT};
use super::board::Board;
use super::hp::with_battery;
use crate::exec::{ExecLimits, Machine};
use crate::load::Aml;
use crate::platform::{initialize, inventory, DeviceInfo};
use crate::region::Space;

/// The capsule's loop: a receive that waits 10 ms (server/run.rs RECV_MS).
const LOOP_MS: u64 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Connect,
    Ini,
    Inventory,
    AcpiMode,
    ReadPower,
}

pub struct Boot {
    pub aml: Aml,
    pub board: Board,
    pub devices: Vec<DeviceInfo>,
    pub power: Power,
    /// Each word reported, with the time it was read.
    pub words: Vec<(u64, u64)>,
    /// The port accesses each step began at.
    pub began: Vec<(Step, u64)>,
    /// The board's clock, in ms, as each step began.
    pub began_ms: Vec<(Step, u64)>,
}

const SPACES: [Space; 4] =
    [Space::SYSTEM_MEMORY, Space::SYSTEM_IO, Space::PCI_CONFIG, Space::EMBEDDED_CONTROL];

impl Boot {
    pub fn new(board: Board) -> Self {
        Self::with(board, true)
    }

    pub fn with(board: Board, battery: bool) -> Self {
        let mut aml = Aml::default();
        let mut b = board;
        let dsdt = crate::enc::table(b"DSDT", 2, &with_battery(battery));
        Machine::new(&mut aml, &mut b, ExecLimits::DEFAULT)
            .load_table(&mut aml, dsdt)
            .expect("loads");
        Boot {
            aml,
            board: b,
            devices: vec![],
            power: Power::default(),
            words: vec![],
            began: vec![],
            began_ms: vec![],
        }
    }

    fn read_power(&mut self) {
        let board = &mut self.board;
        let mut ev = |aml: &mut Aml, id, args| {
            Machine::new(aml, board, ExecLimits::DEFAULT).evaluate(aml, id, args)
        };
        for b in self.power.batteries.iter_mut() {
            let _ = read_info(b, &mut self.aml, &mut ev);
            read_status(b, &mut self.aml, &mut ev);
        }
        for a in self.power.adapters.iter_mut() {
            read_adapter(a, &mut self.aml, &mut ev);
        }
        let now = self.board.now();
        self.power.retry.after(wanted(&self.power), now);
        self.power.read_ms = now;
        self.words.push((now, word(&self.power)));
    }

    pub fn step(&mut self, s: Step) {
        self.began.push((s, self.board.ops));
        self.began_ms.push((s, self.board.now()));
        let board = &mut self.board;
        let mut ev = |aml: &mut Aml, id, args| {
            Machine::new(aml, board, ExecLimits::DEFAULT).evaluate(aml, id, args)
        };
        match s {
            Step::Connect => {
                for space in SPACES {
                    Machine::new(&mut self.aml, &mut self.board, ExecLimits::DEFAULT)
                        .connect(&mut self.aml, space);
                }
            }
            Step::Ini => {
                initialize(&mut self.aml, &mut ev);
            }
            Step::Inventory => self.devices = inventory(&mut self.aml, &mut ev).0,
            Step::AcpiMode => self.board.legacy = false,
            Step::ReadPower => {
                self.power = find(&self.devices);
                self.read_power();
            }
        }
    }

    /// The boot in `order`, then the capsule's loop for `for_ms`.
    pub fn run(board: Board, order: &[Step], for_ms: u64) -> Self {
        Self::run_on(Boot::new(board), order, for_ms)
    }

    pub fn run_on(mut b: Boot, order: &[Step], for_ms: u64) -> Self {
        order.iter().for_each(|s| b.step(*s));
        let end = b.board.now() + for_ms;
        while b.board.now() < end {
            b.board.advance(LOOP_MS);
            let now = b.board.now();
            if b.board.health.retry(now) {
                b.power.read_ms = 0;
            }
            let minute = now.saturating_sub(b.power.read_ms) >= 60_000;
            if minute || b.power.retry.due(now) {
                b.read_power();
            }
        }
        b
    }

    pub fn first(&self) -> u64 {
        self.words[0].1
    }

    /// When the battery was first reported with its charge.
    pub fn charged_at(&self) -> Option<u64> {
        let pc = |w: u64| w & 0xFF != 0xFF;
        self.words.iter().find(|(_, w)| w & BATTERY_PRESENT != 0 && pc(*w)).map(|(t, _)| *t)
    }
}
