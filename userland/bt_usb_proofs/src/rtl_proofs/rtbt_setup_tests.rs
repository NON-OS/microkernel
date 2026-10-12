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

//! A second-layout chip set up end to end against the model ROM: its key
//! id read after its ROM version, then exactly its subsections sent.

use super::fixture_rtbt::{ours, rom, rtbt, FW};
use super::rtl::refused::Refused;
use super::rtl::rtbt::{SECURITY, SNIPPETS};
use super::rtl::setup::{setup, Done};
use nonos_bt_hci::{HciError, Host};
use nonos_bt_wire::WireError;

fn file() -> Vec<u8> {
    let snip = [(1, 15, 0, &[0x11; 300][..]), (2, 15, 0, &[0x22; 600][..])];
    rtbt(&[(SNIPPETS, &snip), (SECURITY, &[(2, 1, 4, &[0x44; 20])]), (8, &[])], 20)
}

#[test]
fn the_key_is_read_and_the_rom_s_subsections_are_sent() {
    let (table, mut m) = ours(&file());
    let mut host = Host::with_budget(rom(), 4);
    let done = setup(&mut host, &mut m, table).unwrap();
    assert!(matches!(done, Done::Patched(c) if c.name == "RTL8852BU"));
    let sim = host.transport();
    assert_eq!(sim.ops[..3], [0x1001, 0xfc6d, 0xfc61]);
    let register = [0x10, 0xa4, 0xad, 0x00, 0xb0];
    assert_eq!(sim.asked_register.as_deref(), Some(&register[..]), "the key id register");
    assert_eq!(sim.image(), vec![0x22; 600], "key 0: no security subsection");
    assert_eq!(sim.frags.len(), 3);
    assert_eq!(m.asked, [FW]);
}

#[test]
fn a_keyed_chip_gets_its_signed_subsection_first() {
    let (table, mut m) = ours(&file());
    let mut sim = rom();
    sim.register = vec![4, 0];
    let mut host = Host::with_budget(sim, 4);
    assert!(setup(&mut host, &mut m, table).is_ok());
    assert_eq!(host.transport().image(), [vec![0x44; 20], vec![0x22; 600]].concat());
}

#[test]
fn a_key_read_refused_or_short_stops_before_the_store() {
    for (refuse, register) in [(Some(0xfc61), vec![0, 0]), (None, vec![0])] {
        let (table, mut m) = ours(&file());
        let mut sim = rom();
        (sim.refuse, sim.register) = (refuse, register);
        let mut host = Host::with_budget(sim, 4);
        let why = setup(&mut host, &mut m, table).err().unwrap();
        assert!(matches!(why, Refused::Hci(_)), "{why:?}");
        if refuse.is_none() {
            assert_eq!(why, Refused::Hci(HciError::Wire(WireError::Length)));
        }
        assert!(host.transport().frags.is_empty() && m.asked.is_empty());
    }
}
