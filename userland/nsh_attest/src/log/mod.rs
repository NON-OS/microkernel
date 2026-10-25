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

//! The TCG event log, crypto-agile format (TCG PC Client Platform Firmware
//! Profile, 10.2), read in full and replayed to the PCRs it extends. The
//! log is the firmware's, carried by the loader: it is what lets a client
//! see which loader PCR 4 holds, since PCR 4 also holds firmware events
//! that differ between machines. It is untrusted until its replay equals
//! the PCR values the quote signed.

mod banks;
mod event;
mod header;
mod le;
mod locality;
mod pcr4;
mod replay;
mod replayed;

pub use banks::Banks;
pub use event::{event, Event};
pub use header::header;
pub use pcr4::{EV_EFI_ACTION, EV_SEPARATOR};
pub use replay::replay;
pub use replayed::Replayed;

/// Logged and never extended. The Spec ID header is one.
pub const EV_NO_ACTION: u32 = 0x0000_0003;
/// A UEFI application the firmware started, measured by its Authenticode
/// digest. The last one in PCR 4 is the loader.
pub const EV_EFI_BOOT_SERVICES_APPLICATION: u32 = 0x8000_0003;
pub const TPM_ALG_SHA256: u16 = 0x000B;
/// The PCRs a PC TPM has.
pub const PCRS: usize = 24;
/// The boot manager's PCR.
pub const PCR_BOOT_MANAGER: usize = 4;
/// Events after the header. A boot logs about a hundred.
pub const MAX_EVENTS: usize = 1024;
/// One event's body. A Secure Boot variable event may carry tens of KB.
pub const MAX_EVENT_BODY: usize = 65_536;
/// Banks a header may declare.
pub const MAX_BANKS: usize = 8;
/// The longest digest of any bank, SHA-512's.
pub const MAX_DIGEST: usize = 64;
