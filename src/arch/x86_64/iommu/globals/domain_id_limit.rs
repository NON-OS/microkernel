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

use super::super::regs::cap::usable_domain_ids;
use super::super::types::MAX_VTD_DOMAINS;
use super::super::unit::report::probed;

/// How many domain ids this boot's units accept. Zero before the probe, so
/// no domain is made for units the kernel has not read.
pub fn domain_id_limit() -> usize {
    probed().map_or(0, |unit| usable_domain_ids(unit.domains, MAX_VTD_DOMAINS))
}
