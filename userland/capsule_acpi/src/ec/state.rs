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

//! The embedded controller the capsule found.

use alloc::collections::BTreeMap;
use nonos_aml::ec::EcPorts;
use nonos_aml::namespace::Id;

pub struct Ec {
    pub ports: EcPorts,
    /// The GPE it raises (_GPE, or the ECDT's); None if neither says.
    pub gpe: Option<u16>,
    /// Its accesses take the Global Lock (_GLK).
    pub glk: bool,
    /// _Qxx by query value.
    pub queries: BTreeMap<u8, Id>,
}
