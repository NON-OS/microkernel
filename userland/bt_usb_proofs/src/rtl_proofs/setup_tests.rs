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

//! The setup against a model ROM: a Realtek ROM in the table is sent its
//! image in order and comes back patched; any other controller, or one
//! patched already, is asked its version and nothing more.

use super::fixture::{chips, epatch, pin, Mem, VERSION};
use super::fixture_image::{CFG, CONFIG, FW};
use super::rom_sim::RomSim;
use super::rtl::chip::Chip;
use super::rtl::setup::{setup, Done};
use nonos_bt_hci::Host;

fn ready() -> (&'static [Chip], Mem, Vec<u8>) {
    let patch: Vec<u8> = (0..600u32).map(|i| (i * 13) as u8).collect();
    let f = epatch(VERSION, &[(1, &[7; 16]), (2, &patch)], 10);
    let table = chips(pin(FW, &f), Some(pin(CFG, CONFIG)), false);
    let mut m = Mem::default();
    m.files.insert(FW, f);
    m.files.insert(CFG, CONFIG.to_vec());
    (table, m, [&patch[..596], &VERSION.to_le_bytes(), CONFIG].concat())
}

#[test]
fn a_rom_in_the_table_is_sent_its_image_and_comes_back_patched() {
    let (table, mut m, want) = ready();
    let mut host = Host::with_budget(RomSim::rtl8821c(), 4);
    let done = setup(&mut host, &mut m, table).map_err(|_| ()).unwrap();
    assert!(matches!(done, Done::Patched(c) if c.name == "RTL8821C"));
    let sim = host.transport();
    assert_eq!(sim.ops, [0x1001, 0xfc6d, 0xfc20, 0xfc20, 0xfc20, 0x1001]);
    assert_eq!(sim.image(), want);
    let idx: Vec<u8> = sim.frags.iter().map(|(i, _)| *i).collect();
    assert_eq!(idx, [0, 1, 0x82]);
}

fn untouched(sim: RomSim) {
    let (table, mut m, _) = ready();
    let mut host = Host::with_budget(sim, 4);
    let done = setup(&mut host, &mut m, table).map_err(|_| ()).unwrap();
    assert!(matches!(done, Done::NotOurs));
    assert_eq!(host.transport().ops, [0x1001], "asked its version, nothing more");
    assert!(m.asked.is_empty(), "the store is not read");
}

#[test]
fn another_companys_controller_is_left_alone() {
    untouched(RomSim { company: 0x000f, ..RomSim::rtl8821c() });
}

#[test]
fn a_controller_already_patched_is_left_alone() {
    untouched(RomSim { lmp_subversion: 0x75b8, ..RomSim::rtl8821c() });
}

#[test]
fn a_realtek_rom_not_in_the_table_is_left_alone() {
    untouched(RomSim { hci_revision: 0x000d, ..RomSim::rtl8821c() });
    untouched(RomSim { hci_version: 9, ..RomSim::rtl8821c() });
}
