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

//! The few vendor bits a driver of a claimed device may flip beyond Command
//! and MSI-X, each the exact bits Linux's driver for that class writes and
//! nothing beside them. Pure, so kernel_proofs holds every rule.
//!
//! - Intel HD Audio (class 04, subclass 03 or 01 for a DSP-mode part):
//!   TCSEL at 0x44 bits 0-2, CGCTL MISCBDCGE at 0x48 bit 6 (cleared around
//!   the controller reset on Skylake and later, Gemini Lake included) and
//!   DEVC NOSNOOP at 0x78 bit 11 (snd_hda_intel's azx_init_pci).
//! - AMD/ATI HD Audio: the snoop bits at 0x42 bits 0-2.
//! - A network controller (class 02): PCIe Device Control 2's completion
//!   timeout value and disable, bits 0-4 at the PCIe capability + 0x28, which
//!   rtw88 and iwlwifi set on real silicon.
//! - The Intel SMBus host (class 0C, subclass 05): HOSTC at 0x40, its host
//!   enable (bit 0) and I2C mode (bit 2), as i2c-i801's probe writes them.
//! - A UHCI controller (class 0C, subclass 03, interface 00), any vendor:
//!   LEGSUP at 0xC0, its firmware trap and SMI enables (7:0), their status
//!   (11:8, 15, each cleared by a one) and PIRQD routing (13), so a polled
//!   driver turns firmware's legacy keyboard emulation off. Bits 12 and 14
//!   stay as read (Intel PIIX4 and ICH datasheets, as recalled, unverified).
//!
//! Anything else stays refused: these registers hold no address, no BAR, no
//! routing, and change nothing about what the device may reach.

mod ident;
mod writable;

// Public for kernel_proofs, which mounts this module at its root; the
// kernel itself holds it crate-private.
#[allow(unreachable_pub)]
pub use ident::Ident;
#[allow(unreachable_pub)]
pub use writable::{only, writable};
