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

/// Interface identity. The low nibble says which register file the part
/// presents; a CRB part reads back zero at the FIFO identity offset, so
/// identity must come from whichever file is actually there.
pub(super) const TPM_INTERFACE_ID: u32 = 0x0030;
pub(super) const TPM_INTF_TYPE_MASK: u32 = 0x0F;
pub(super) const TPM_INTF_TYPE_CRB: u32 = 0x1;

/// Locality request and its result. A CRB part takes requests here; the FIFO
/// offset 0x00 is read-only `TPM_LOC_STATE` on CRB, so a FIFO-style request
/// writes into a register that cannot grant anything.
pub(super) const TPM_LOC_CTRL: u32 = 0x0008;
pub(super) const TPM_LOC_STS: u32 = 0x000C;
pub(super) const TPM_LOC_CTRL_REQUEST: u32 = 0x01;
pub(super) const TPM_LOC_CTRL_RELINQUISH: u32 = 0x02;
pub(super) const TPM_LOC_STS_GRANTED: u32 = 0x01;

/// Control registers, at their offset from the control area (`area.rs`),
/// which Intel's part puts at 0x40 in the register window and AMD's in memory
/// of its own. A CRB part takes no command bytes through a port: it publishes
/// a buffer in memory and a doorbell.
pub(super) const TPM_CRB_CTRL_REQ: u32 = 0x0000;
pub(super) const TPM_CRB_CTRL_STS: u32 = 0x0004;
pub(super) const TPM_CRB_CTRL_START: u32 = 0x000C;
pub(super) const TPM_CRB_CTRL_CMD_SIZE: u32 = 0x0018;
pub(super) const TPM_CRB_CTRL_CMD_LADDR: u32 = 0x001C;
pub(super) const TPM_CRB_CTRL_CMD_HADDR: u32 = 0x0020;
pub(super) const TPM_CRB_CTRL_RSP_SIZE: u32 = 0x0024;
pub(super) const TPM_CRB_CTRL_RSP_LADDR: u32 = 0x0028;
pub(super) const TPM_CRB_CTRL_RSP_HADDR: u32 = 0x002C;

pub(super) const TPM_CRB_REQ_COMMAND_READY: u32 = 0x01;
pub(super) const TPM_CRB_STS_TPM_IDLE: u32 = 0x02;
pub(super) const TPM_CRB_START_GO: u32 = 0x01;

/// Waits by the clock, not by a count of register reads: a read of a real
/// part takes microseconds, not the nanoseconds of an emulated one, so a count
/// tuned in QEMU ran out on silicon while the part was still working. Ready
/// within two seconds (TCG PTP's TIMEOUT_B); a command within two minutes, as
/// long as a key the part creates can take on a slow discrete TPM.
pub(super) const READY_MS: u64 = 2_000;
pub(super) const COMPLETE_MS: u64 = 120_000;
/// Locality requests are answered in microseconds; a whole second is generous.
pub(super) const LOCALITY_MS: u64 = 1_000;
/// Largest buffer this driver will believe. The size comes from a part-supplied
/// register, so an implausible value would otherwise become the length of a
/// copy into an arbitrary physical address.
pub(super) const MAX_PLAUSIBLE_BUFFER: u32 = 0x1_0000;
