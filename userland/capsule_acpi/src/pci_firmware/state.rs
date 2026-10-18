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

use alloc::vec::Vec;
use nonos_aml::namespace::Id;
use nonos_aml::pci_dsm::{HostFacts, Label};

pub type Bdf = (u16, u8, u8, u8);

pub struct Host {
    pub id: Id,
    pub segment: u16,
    /// The buses the bridge decodes, from its `_BBN` and `_CRS`.
    pub buses: (u8, u8),
    pub facts: HostFacts,
}

pub struct Labelled {
    pub bdf: Bdf,
    pub label: Label,
}

#[derive(Default)]
pub struct PciFirmware {
    pub hosts: Vec<Host>,
    pub labels: Vec<Labelled>,
}

impl PciFirmware {
    /// The host bridge a function sits below.
    pub fn host_of(&self, f: Bdf) -> Option<&Host> {
        self.hosts.iter().find(|h| h.segment == f.0 && (h.buses.0..=h.buses.1).contains(&f.1))
    }
}
