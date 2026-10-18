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

use super::phys::Phys;
use alloc::vec::Vec;
use nonos_aml::apei::Ghes;
use nonos_aml::namespace::Id;

pub struct Source {
    pub id: u16,
    pub ghes: Ghes,
    /// For a polled source, when to look next.
    pub next_ms: u64,
}

#[derive(Default)]
pub struct Apei {
    pub sources: Vec<Source>,
    pub phys: Phys,
    /// PNP0C33 devices: a Notify on one means an SCI source has an error.
    pub error_devices: Vec<Id>,
}
