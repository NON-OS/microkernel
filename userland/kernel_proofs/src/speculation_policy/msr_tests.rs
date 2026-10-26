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

//! The MSRs the mitigations write and the bits they read, as the Intel SDM
//! numbers them (vol. 4, table 2-2): IA32_SPEC_CTRL 48h, IA32_PRED_CMD 49h,
//! IA32_ARCH_CAPABILITIES 10Ah and IA32_FLUSH_CMD 10Bh.

use super::constants::{ARCH_CAP_MDS_NO, ARCH_CAP_RDCL_NO, ARCH_CAP_SBDR_SSDP_NO};
use super::constants::{ARCH_CAP_SSB_NO, ARCH_CAP_TAA_NO, FLUSH_CMD_L1D, PRED_CMD_IBPB};
use super::constants::{MSR_IA32_ARCH_CAPABILITIES, MSR_IA32_FLUSH_CMD};
use super::constants::{MSR_IA32_PRED_CMD, MSR_IA32_SPEC_CTRL};

#[test]
fn the_msrs_are_the_sdms() {
    assert_eq!((MSR_IA32_SPEC_CTRL, MSR_IA32_PRED_CMD), (0x48, 0x49));
    assert_eq!((MSR_IA32_ARCH_CAPABILITIES, MSR_IA32_FLUSH_CMD), (0x10A, 0x10B));
}

#[test]
fn the_command_and_capability_bits_are_the_sdms() {
    assert_eq!((PRED_CMD_IBPB, FLUSH_CMD_L1D), (1, 1));
    assert_eq!((ARCH_CAP_RDCL_NO, ARCH_CAP_SSB_NO, ARCH_CAP_MDS_NO), (1, 1 << 4, 1 << 5));
    assert_eq!((ARCH_CAP_TAA_NO, ARCH_CAP_SBDR_SSDP_NO), (1 << 8, 1 << 13));
}
