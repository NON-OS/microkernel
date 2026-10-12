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

//! What the capsule's host keeps between accesses.

use super::ports::Ports;
use super::windows::Windows;
use nonos_aml::ec::EcPorts;
use nonos_aml::events::Pm1Cnt;

#[derive(Default)]
pub struct CapsuleHost {
    pub windows: Windows,
    pub ports: Ports,
    /// The FACS, mapped, for the Global Lock; None on a platform without
    /// one, where the lock is the capsule's alone.
    pub facs: Option<u64>,
    /// PM1 control, for GBL_RLS; None until the events are up.
    pub pm1_cnt: Option<Pm1Cnt>,
    /// Region accesses the host refused or that failed, for the log.
    pub refused: u64,
    /// The embedded controller, once found, and whether its accesses take
    /// the Global Lock (_GLK, ACPI 6.5, 6.5.7).
    pub ec: Option<(EcPorts, bool)>,
    /// Whether the capsule holds the Global Lock now, through \_GL or a
    /// Lock field, so an EC access inside it does not ask again.
    pub lock_held: bool,
}
