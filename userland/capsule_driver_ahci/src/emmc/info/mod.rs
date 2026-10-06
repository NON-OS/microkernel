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

//! Reply payloads that describe the device, in the AHCI capsule's wire
//! layout (little endian throughout). Pure, so the layouts are pinned on
//! the host.
//!
//! IDENTIFY is the AHCI capsule's op 8 and its 76-byte reply (see the
//! capsule's protocol/identify_reply.rs), with byte 14, the medium, set to
//! 1 for an eMMC part. `names` gives the model and serial it carries: the
//! manufacturer, when its JEDEC id is one of the common ones, and the CID
//! product name; the CID serial number as eight hex digits.
//!
//! CONTROLLER_INFO keeps AHCI's 24 bytes, with SDHCI registers where AHCI
//! has its own: CAP = Capabilities (0x40), GHC = Host Control 1 in 7:0,
//! Power Control in 15:8, Clock Control in 31:16, PI = 1 (slot 0), VS =
//! Host Controller Version (0xFE), CAP2 = Capabilities (0x44), port count 1.
//!
//! PORT_LIST answers one 36-byte entry: index 0, implemented 1, present 1
//! when the card is up, kind 5 (eMMC; AHCI uses 0 to 4 and 255), then SSTS
//! = Present State, SIG = the card's OCR, IS = Normal and Error Interrupt
//! Status, CMD = the card's last R1 status, TFD 0, SERR = Error Interrupt
//! Status, SACT 0, CI 0.

mod controller_info;
mod layout;
mod maker;
mod names;
mod port_entry;

pub use controller_info::controller_info;
pub use layout::{
    CONTROLLER_INFO_LEN, MEDIUM_EMMC, MODEL_MAX, PORT_ENTRY_LEN, PORT_KIND_EMMC, SERIAL_LEN,
};
pub use names::{names, Names};
pub use port_entry::port_entry;
