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

use super::port::Port;
use crate::constants::ata::{ATA_IDENTIFY, DATA_BUF_BYTES};
use crate::constants::identify::IDENTIFY_WORDS;
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

// The data buffer holds the whole IDENTIFY block, so the copy below stays in it.
const _: () = assert!(IDENTIFY_WORDS * 2 <= DATA_BUF_BYTES as usize);

pub fn identify(port: &mut Port, regs: Regs) -> AhciResult<u64> {
    super::build::build_slot0(port, ATA_IDENTIFY, 0, 1, false)?;
    super::issue::run(port, regs)?;
    let block = read_block(port)?;
    let sectors = crate::identity::capacity(&block).map_err(AhciError::IdentityRefused)?;
    port.capacity_sectors = sectors;
    port.names = crate::identity::names(&block);
    Ok(sectors)
}

/// Copy the drive's IDENTIFY block out of the data buffer, so every rule in
/// `identity` judges one fixed copy rather than memory the device can still
/// write. The words are little-endian (ATA8-ACS, 3.2.6).
fn read_block(port: &Port) -> AhciResult<[u16; IDENTIFY_WORDS]> {
    let mut bytes = [0u8; IDENTIFY_WORDS * 2];
    port.take(&mut bytes)?;
    let mut words = [0u16; IDENTIFY_WORDS];
    for (w, b) in words.iter_mut().zip(bytes.chunks_exact(2)) {
        *w = u16::from_le_bytes([b[0], b[1]]);
    }
    Ok(words)
}
