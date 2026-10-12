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

//! The capsule's table: each ROM is known by one answer, only Realtek's
//! company identifier matches, and chips that share a config share its pin.

use super::rtl::chip::{rom, REALTEK};
use super::rtl::chips::CHIPS;
use super::rtl::pins_config::RTL8821C_CONFIG;
use super::rtl::version::Version;

fn answer(c: &super::rtl::chip::Chip, company: u16) -> Version {
    let (hci_version, hci_revision) = (c.hci_version, c.hci_revision);
    Version { hci_version, hci_revision, company, lmp_subversion: c.lmp_subversion }
}

#[test]
fn each_rom_answer_names_one_chip() {
    for (i, a) in CHIPS.iter().enumerate() {
        assert_eq!(rom(CHIPS, &answer(a, REALTEK)).map(|c| c.name), Some(a.name));
        for b in &CHIPS[i + 1..] {
            assert_ne!(a.name, b.name);
            assert_ne!((a.lmp_subversion, a.hci_revision), (b.lmp_subversion, b.hci_revision));
            assert_ne!(a.project, b.project);
            assert_ne!(a.firmware.file, b.firmware.file);
        }
    }
}

#[test]
fn only_realteks_company_identifier_matches() {
    for c in CHIPS {
        assert!(rom(CHIPS, &answer(c, 0x000f)).is_none(), "{}", c.name);
        assert!(rom(CHIPS, &answer(c, 0x0002)).is_none(), "{}", c.name);
    }
    assert_eq!(REALTEK, 0x005d);
}

#[test]
fn every_chip_has_a_config_and_shared_configs_share_a_pin() {
    let shared: Vec<&str> =
        CHIPS.iter().filter(|c| c.config == Some(&RTL8821C_CONFIG)).map(|c| c.name).collect();
    assert_eq!(shared, ["RTL8723D", "RTL8821A", "RTL8821C"]);
    assert!(CHIPS.iter().all(|c| c.config.is_some()));
    let needed: Vec<&str> = CHIPS.iter().filter(|c| c.config_needed).map(|c| c.name).collect();
    assert_eq!(needed, ["RTL8723D", "RTL8822B"]);
}
