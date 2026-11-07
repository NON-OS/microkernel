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

//! The machine under the reproduction: the modelled EC on its two ports;
//! firmware's SMI handler, which in legacy mode (before ACPI mode is
//! entered) services the EC's events itself on the same ports, at the port
//! accesses a schedule names; and the EC's own start, which leaves it
//! answering nothing until `ready_ms`. The capsule's side of each EC byte
//! is capsule_acpi's host/ec.rs: refused while `Health` says off, the
//! handshake (`ec::read`/`ec::write`), the answer counted in `Health`.

use crate::ec::{read, write, Health, QR_EC};
use crate::ec_model::{EcModel, PORTS};
use crate::events::EventIo;
use crate::exec::{Host, RegionAccess};
use crate::object::Object;
use crate::region::Space;
use std::collections::BTreeSet;

pub struct Board {
    pub ec: EcModel,
    pub health: Health,
    /// Firmware still owns the EC's events: ACPI mode not yet entered.
    pub legacy: bool,
    /// The port accesses, counted from 1, before which an SMI runs.
    pub smi_at: BTreeSet<u64>,
    pub ready_ms: u64,
    pub ops: u64,
    /// SMIs that found an event and ran their own QR_EC.
    pub smis: u32,
    lock: u32,
}

impl Board {
    pub fn new(ready_ms: u64, smi_at: BTreeSet<u64>) -> Self {
        let mut ec = EcModel::new(2);
        // The events an EC queues at power-on (AC, battery, thermal), one
        // per SMI the schedule may run.
        ec.events.extend((0..smi_at.len()).map(|i| 0x50 + i as u8));
        Board {
            ec,
            health: Health::default(),
            legacy: true,
            smi_at,
            ready_ms,
            ops: 0,
            smis: 0,
            lock: 0,
        }
    }

    pub fn now(&self) -> u64 {
        self.ec.slept_ms
    }

    pub fn advance(&mut self, ms: u64) {
        self.ec.slept_ms += ms;
    }

    /// Firmware's SMI handler: QR_EC on the shared ports and the answer
    /// read away. It replaces whatever byte the EC held for the capsule.
    fn smm(&mut self) {
        if self.ec.events.is_empty() {
            return;
        }
        self.ec.take(0x66, QR_EC as u8);
        self.ec.out.take();
        self.ec.busy = 0;
        self.smis += 1;
    }

    fn step(&mut self) {
        self.ops += 1;
        if self.legacy && self.smi_at.contains(&self.ops) {
            self.smm();
        }
        self.ec.stuck = self.now() < self.ready_ms;
    }

    pub fn mem(&mut self, at: u8, v: u8) {
        self.ec.mem[at as usize] = v;
    }
}

impl EventIo for Board {
    fn read(&mut self, port: u16, width: u8) -> Option<u16> {
        self.step();
        self.ec.read(port, width)
    }
    fn write(&mut self, port: u16, width: u8, value: u16) -> bool {
        self.step();
        self.ec.write(port, width, value)
    }
    fn sleep(&mut self, ms: u64) {
        self.advance(ms);
    }
    fn now_ms(&mut self) -> u64 {
        self.now()
    }
}

impl Host for Board {
    fn sleep(&mut self, ms: u64) {
        self.advance(ms);
    }
    fn stall(&mut self, _: u64) {}
    fn timer(&mut self) -> u64 {
        self.now() * 10_000
    }
    fn debug(&mut self, _: &Object) {}
    fn fatal(&mut self, _: u8, _: u32, _: u64) {}
    fn region_read(&mut self, a: &RegionAccess<'_>) -> Option<u64> {
        if a.space != Space::EMBEDDED_CONTROL {
            return Some(0);
        }
        if !self.health.usable() || a.width != 1 {
            return None;
        }
        let r = read(&PORTS, self, u8::try_from(a.address).ok()?);
        self.health.note(&r);
        r.ok().map(u64::from)
    }
    fn region_write(&mut self, a: &RegionAccess<'_>, v: u64) -> bool {
        if a.space != Space::EMBEDDED_CONTROL {
            return true;
        }
        let Ok(at) = u8::try_from(a.address) else { return false };
        if !self.health.usable() || a.width != 1 {
            return false;
        }
        let r = write(&PORTS, self, at, v as u8);
        self.health.note(&r);
        r.is_ok()
    }
    fn transaction(
        &mut self,
        _: &RegionAccess<'_>,
        _: [u8; 3],
        _: Option<&[u8]>,
        _: &mut Vec<u8>,
    ) -> bool {
        false
    }
    fn gpio(&mut self, _: &[u8], _: u32, _: u32, _: Option<u64>) -> Option<u64> {
        None
    }
    fn table(&mut self, _: &[u8], _: &[u8], _: &[u8]) -> Option<Vec<u8>> {
        None
    }
    fn admit_table(&mut self, _: &[u8]) -> bool {
        false
    }
    fn lock_word(&mut self, old: u32, new: u32) -> Option<u32> {
        let found = self.lock;
        if found == old {
            self.lock = new;
        }
        Some(found)
    }
    fn lock_released(&mut self) {}
}
