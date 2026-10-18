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

//! The modelled host and device: QEMU-like by default, and each test's knobs.

use super::lu::Lu;

pub struct Cfg {
    pub cap: u32,
    pub ver: u32,
    /// HCE never reads back 1; with `hce_stuck_on`, never back 0.
    pub hce_never: bool,
    pub hce_stuck_on: bool,
    /// HCE reads 0 this many times after it is set, as a host initialising.
    pub hce_settle: u32,
    /// The first this many DME_LINKSTARTUPs answer result 1.
    pub link_fails: u32,
    pub no_device: bool,
    /// PA_LOCAL_TX_LCC_ENABLE as the host starts.
    pub lcc: u32,
    pub lists_never: bool,
    /// NOP OUT never completes.
    pub nop_hang: bool,
    /// The first this many NOP OUTs are answered with a RESPONSE UPIU.
    pub nop_wrong: u32,
    /// fDeviceInit reads 1 this many times after it is set.
    pub init_reads: u32,
    /// Every SCSI command is left in the doorbell.
    pub scsi_hang: bool,
    /// A UTRLCLR is recorded but never takes: the slot stays the host's.
    pub clear_never: bool,
    /// The OCS the host ends every SCSI command with; 0 is success.
    pub scsi_ocs: u8,
    pub lus: Vec<Lu>,
    /// Further REPORT LUNS entries, as given.
    pub extra_entries: Vec<[u8; 8]>,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            // 64-bit addressing, 32 slots (NUTRS 31), UFSHCI 4.0.
            cap: 1 << 24 | 31,
            ver: 0x0400,
            hce_stuck_on: false,
            hce_never: false,
            hce_settle: 2,
            link_fails: 0,
            no_device: false,
            lcc: 0,
            lists_never: false,
            nop_hang: false,
            nop_wrong: 0,
            init_reads: 3,
            scsi_hang: false,
            clear_never: false,
            scsi_ocs: 0,
            lus: vec![Lu::new(0, 4096, 1000)],
            extra_entries: Vec::new(),
        }
    }
}
