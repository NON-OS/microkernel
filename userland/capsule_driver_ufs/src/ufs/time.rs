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

//! Every wait, in milliseconds. Each is Linux ufshcd's bound or more, so
//! this driver never gives up on a part sooner than Linux would; the error
//! lines that quote a bound are held to these in ufs_proofs.

/// ufshcd_hba_stop: HCE must read 0 within 1 ms; ten are allowed.
pub const HCE_OFF_MS: u64 = 10;
/// ufshcd_hba_execute_hce: 1 ms, then 50 polls of usleep_range(1000, 1100).
pub const HCE_ON_MS: u64 = 1 + 50 * 11 / 10;
/// UIC_CMD_TIMEOUT_DEFAULT, for UCRDY and for each command's completion.
pub const UIC_MS: u64 = 500;
/// NOP_OUT_TIMEOUT, per NOP OUT.
pub const NOP_MS: u64 = 50;
/// QUERY_REQ_TIMEOUT, per query.
pub const QUERY_MS: u64 = 1500;
/// FDEVICEINIT_COMPL_TIMEOUT: fDeviceInit must clear within this.
pub const DEVICE_INIT_MS: u64 = 1500;
/// ufshcd_clear_cmd: a request cleared through UTRLCLR must leave the
/// doorbell within a second.
pub const CLEAR_MS: u64 = 1000;
/// A SCSI command, as sd's SD_TIMEOUT (30 s).
pub const SCSI_MS: u64 = 30_000;
/// A logical unit coming ready after a reset or power-on: TEST UNIT READY
/// is retried while it reports NOT READY or UNIT ATTENTION, this long. Linux
/// leaves that to the SCSI midlayer's retries; five seconds is this driver's.
pub const UNIT_READY_MS: u64 = 5000;
