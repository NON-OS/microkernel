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

/*
 * What every CPU keeps in SPEC_CTRL, when the kernel writes IBRS again on
 * entry, and when a switch takes a prediction barrier. The MSR bit constants
 * and the pure policy are included by path.
 */

#[path = "../../../../src/security/hardening/spectre_mitigations/constants.rs"]
#[allow(dead_code)]
pub mod constants;
#[path = "../../../../src/security/hardening/spectre_mitigations/policy.rs"]
pub mod policy;
mod tests;
