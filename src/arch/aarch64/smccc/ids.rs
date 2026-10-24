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

//! Function identifiers from SMCCC (DEN0028), the Arm architecture service
//! range: fast calls, owning entity 0, so bit 31 set and nothing else in
//! 30:16. Recalled from the specification, not checked against a copy in
//! this tree; the letter lists them for the owner.

/// SMCCC_VERSION, discovered through PSCI_FEATURES (DEN0028, "SMCCC_VERSION").
pub(super) const VERSION: u32 = 0x8000_0000;
/// SMCCC_ARCH_FEATURES: whether, and how, a function in this range applies.
pub(super) const ARCH_FEATURES: u32 = 0x8000_0001;
/// Spectre variant 2 (CVE-2017-5715): invalidate the branch predictor.
pub const ARCH_WORKAROUND_1: u32 = 0x8000_8000;
/// Speculative store bypass (CVE-2018-3639): turn the PE's mitigation on.
pub const ARCH_WORKAROUND_2: u32 = 0x8000_7FFF;
/// Spectre-BHB (CVE-2022-23960), which also does what _1 does.
pub const ARCH_WORKAROUND_3: u32 = 0x8000_3FFF;

/// SMCCC 1.1 as SMCCC_VERSION returns it: major in 30:16, minor in 15:0.
/// 1.1 is where SMCCC_ARCH_FEATURES and the workarounds begin.
pub(super) const V1_1: u32 = 0x0001_0001;
