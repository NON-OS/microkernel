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

//! What stops the setup: a file not pinned or not there, before anything
//! is downloaded, and a command the ROM refuses.

use super::fixture::{epatch, Mem, VERSION};
use super::fixture_setup::{ours, run, FW};
use super::rom_sim::RomSim;
use super::rtl::rom::{DOWNLOAD, READ_ROM_VERSION};
use super::rtl::{chips::CHIPS, refused::Refused};
use nonos_bt_hci::HciError;

#[test]
fn the_real_pins_refuse_a_file_that_is_not_the_published_one() {
    let mut m = Mem::default();
    m.files.insert(FW, epatch(VERSION, &[(2, &[0; 64])], 10));
    let (why, ops) = run(RomSim::rtl8821c(), CHIPS, &mut m);
    assert_eq!(why, Refused::NotPinned(FW));
    assert_eq!(ops, [0x1001, 0xfc6d], "nothing downloaded");
    let (why, ops) = run(RomSim::rtl8821c(), CHIPS, &mut Mem::default());
    assert_eq!((why, ops), (Refused::Missing(FW), vec![0x1001, 0xfc6d]));
}

#[test]
fn a_command_the_rom_refuses_ends_the_setup_there() {
    let (table, mut m) = ours();
    let sim = RomSim { refuse: Some(0xfc20), ..RomSim::rtl8821c() };
    let (why, ops) = run(sim, table, &mut m);
    assert_eq!(why, Refused::Hci(HciError::Status(DOWNLOAD, 0x12)));
    assert_eq!(ops, [0x1001, 0xfc6d, 0xfc20], "no fragment after the refused one");
    let (table, mut m) = ours();
    let sim = RomSim { refuse: Some(0xfc6d), ..RomSim::rtl8821c() };
    let (why, ops) = run(sim, table, &mut m);
    assert_eq!(why, Refused::Hci(HciError::Status(READ_ROM_VERSION, 0x12)));
    assert_eq!((ops, m.asked.len()), (vec![0x1001, 0xfc6d], 0));
}
