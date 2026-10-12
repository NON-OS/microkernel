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

//! The FADT's event hardware: the SCI, the ACPI mode switch, the PM1 event
//! and control blocks and the two GPE blocks (ACPI 6.5, 5.2.9).

use super::bytes::{u32_at, u8_at};
use super::fadt_fields as f;
use super::gas::{block, gas, Block, BlockError};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fadt {
    pub sci: u16,
    pub smi_cmd: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub pm1a_evt: Option<Block>,
    pub pm1b_evt: Option<Block>,
    pub pm1a_cnt: Option<Block>,
    pub pm1b_cnt: Option<Block>,
    pub gpe0: Option<Block>,
    pub gpe1: Option<Block>,
    pub gpe1_base: u8,
    pub hw_reduced: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FadtError {
    /// Not "FACP", or shorter than the first FADT.
    Table,
    Block(BlockError),
}

/// The table as the kernel handed it, header included and already checked.
pub fn parse(t: &[u8]) -> Result<Fadt, FadtError> {
    if t.len() < f::MIN_LEN || t[..4] != *b"FACP" {
        return Err(FadtError::Table);
    }
    let blk = |x: usize, legacy: usize, len: u8| {
        block(gas(t, x), u32_at(t, legacy), len).map_err(FadtError::Block)
    };
    let (evt, cnt) = (u8_at(t, f::PM1_EVT_LEN), u8_at(t, f::PM1_CNT_LEN));
    Ok(Fadt {
        sci: u16::from_le_bytes([t[f::SCI_INT], t[f::SCI_INT + 1]]),
        smi_cmd: u32_at(t, f::SMI_CMD),
        acpi_enable: u8_at(t, f::ACPI_ENABLE),
        acpi_disable: u8_at(t, f::ACPI_DISABLE),
        pm1a_evt: blk(f::X_PM1A_EVT_BLK, f::PM1A_EVT_BLK, evt)?,
        pm1b_evt: blk(f::X_PM1B_EVT_BLK, f::PM1B_EVT_BLK, evt)?,
        pm1a_cnt: blk(f::X_PM1A_CNT_BLK, f::PM1A_CNT_BLK, cnt)?,
        pm1b_cnt: blk(f::X_PM1B_CNT_BLK, f::PM1B_CNT_BLK, cnt)?,
        gpe0: blk(f::X_GPE0_BLK, f::GPE0_BLK, u8_at(t, f::GPE0_BLK_LEN))?,
        gpe1: blk(f::X_GPE1_BLK, f::GPE1_BLK, u8_at(t, f::GPE1_BLK_LEN))?,
        gpe1_base: u8_at(t, f::GPE1_BASE),
        hw_reduced: u32_at(t, f::FLAGS) & f::HW_REDUCED_ACPI != 0,
    })
}
