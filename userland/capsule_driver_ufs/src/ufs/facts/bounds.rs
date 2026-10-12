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

//! Every wait and retry count, in milliseconds and tries. JESD223 and
//! JESD220 set few of these as recalled; where a value was seen in another
//! driver it says so and is unverified, and where it is this engine's own
//! choice it says why.

/// HCE written 0 must read 0: 1 ms observed in Linux ufshcd.c, unverified;
/// ten allowed here.
pub const HCE_OFF_MS: u64 = 10;
/// HCE written 1 must read 1 again once the host has initialised. This
/// engine's own bound: generous, as nothing else can run meanwhile.
pub const HCE_ON_MS: u64 = 100;
/// HCE may read 1, 0, 1 while the host initialises (observed in Linux
/// ufshcd.c, unverified), so its first read waits this long.
pub const HCE_FIRST_READ_MS: u64 = 1;
/// UCRDY, and each UIC command's completion: 500 ms observed in Linux
/// ufshcd.c, unverified.
pub const UIC_MS: u64 = 500;
/// Link startups after the first, each after a host reset: 3 observed in
/// Linux ufshcd.c, unverified.
pub const LINK_RETRIES: usize = 3;
/// One NOP OUT: 50 ms, and up to five sent; observed in Linux ufshcd.c,
/// unverified.
pub const NOP_MS: u64 = 50;
pub const NOP_TRIES: usize = 5;
/// One query: 1500 ms, up to three sent; observed in Linux, unverified.
pub const QUERY_MS: u64 = 1500;
pub const QUERY_TRIES: usize = 3;
/// fDeviceInit must clear within this: 1500 ms observed in Linux,
/// unverified. It is read a millisecond apart.
pub const DEVICE_INIT_MS: u64 = 1500;
pub const DEVICE_INIT_POLL_MS: u64 = 1;
/// A slot cleared through UTRLCLR must leave the doorbell within this: one
/// second observed in Linux ufshcd.c, unverified.
pub const CLEAR_MS: u64 = 1000;
/// A SCSI command: 30 s observed in Linux sd.c, unverified.
pub const SCSI_MS: u64 = 30_000;
/// A logical unit coming ready after reset: TEST UNIT READY retried while
/// it reports NOT READY or UNIT ATTENTION, this long. This engine's own.
pub const UNIT_READY_MS: u64 = 5000;
