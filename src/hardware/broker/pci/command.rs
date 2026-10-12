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

//! The PCI Command register half of the `MkPciConfigWrite` allowlist.

use super::command_bits::{exact, plain};
use super::types::{PciWriteError, WriteAction};

/*
 * The only PCI Command bits a driver capsule may flip. Bus Master is for
 * DMA. Memory Space decodes the MMIO BAR: firmware often leaves it clear on
 * an LPSS controller it did not use, and then every MMIO access silently
 * drops. Interrupt Disable lets a driver that polls its device silence the
 * legacy INTx pin: a level-triggered line is shared on real chipsets and on
 * QEMU's q35, and a device whose status no driver reads holds it up, so
 * every other device on the line sees deliveries it never raised. The bit
 * quiets only the claimed device; MSI and MSI-X are unaffected. No other
 * command bit (I/O space, SERR, parity, etc.) is writable through this path.
 * The bits themselves are in `command_bits.rs`, pure for the proofs.
 */
/// `masked` is the form of `change.rs`: `new` is then the whole register.
pub(super) fn validate_command(
    new: u16,
    current: u16,
    masked: bool,
) -> Result<WriteAction, PciWriteError> {
    let desired = if masked { exact(new, current) } else { plain(new, current) };
    desired.map(WriteAction::Command).ok_or(PciWriteError::BitsNotAllowed)
}
