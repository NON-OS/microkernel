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

//! A Realtek ROM as this setup meets it: it answers Read Local Version with
//! its ROM's numbers until the last fragment is in, Read ROM Version with
//! its ROM version, the key id register with `register`, and each Download
//! with the index it took. It keeps every command's opcode and every
//! fragment, so a proof can read back what reached it.

use std::collections::VecDeque;

pub struct RomSim {
    pub company: u16,
    pub lmp_subversion: u16,
    pub hci_revision: u16,
    pub hci_version: u8,
    pub rom: u8,
    /// What Read Local Version answers once the patch runs; `None` for a
    /// controller that takes every fragment and still runs its ROM.
    pub patched: Option<u16>,
    /// A command answered with status 0x12, Invalid HCI Command Parameters.
    pub refuse: Option<u16>,
    /// Download answered with no index octet after the status.
    pub terse: bool,
    /// The two octets of the key id register, key id first; a proof may
    /// leave one out to answer short.
    pub register: Vec<u8>,
    /// The parameters of the last register read.
    pub asked_register: Option<Vec<u8>>,
    pub ops: Vec<u16>,
    pub frags: Vec<(u8, Vec<u8>)>,
    pub(super) events: VecDeque<Vec<u8>>,
}

impl RomSim {
    /// An RTL8821C's ROM, version 1, as the capsule's table knows it.
    pub fn rtl8821c() -> Self {
        Self {
            company: 0x005d,
            lmp_subversion: 0x8821,
            hci_revision: 0x000c,
            hci_version: 8,
            rom: 1,
            patched: Some(0x75b8),
            refuse: None,
            terse: false,
            register: vec![0, 0],
            asked_register: None,
            ops: Vec::new(),
            frags: Vec::new(),
            events: VecDeque::new(),
        }
    }

    /// The octets downloaded, in the order they came.
    pub fn image(&self) -> Vec<u8> {
        self.frags.iter().flat_map(|(_, d)| d.iter().copied()).collect()
    }
}
