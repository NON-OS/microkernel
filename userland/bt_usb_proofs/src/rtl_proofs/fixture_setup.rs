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

//! A setup run to its refusal against a model ROM, and a patch file of
//! three fragments pinned in a one-chip table.

use super::fixture::{chips, epatch, pin, Mem, VERSION};
use super::rom_sim::RomSim;
use super::rtl::{chip::Chip, refused::Refused, setup::setup};
use nonos_bt_hci::Host;

pub const FW: &str = "rtl8821c_fw.bin";

pub fn run(sim: RomSim, table: &'static [Chip], m: &mut Mem) -> (Refused, Vec<u16>) {
    let mut host = Host::with_budget(sim, 4);
    let why = setup(&mut host, m, table).err().unwrap();
    (why, host.transport().ops.clone())
}

pub fn ours() -> (&'static [Chip], Mem) {
    let f = epatch(VERSION, &[(2, &[5; 700])], 10);
    let table = chips(pin(FW, &f), None, false);
    let mut m = Mem::default();
    m.files.insert(FW, f);
    (table, m)
}
