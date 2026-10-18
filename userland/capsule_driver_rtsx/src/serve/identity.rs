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

//! CAPACITY and IDENTIFY. The card's serial is told only to a sender
//! other than the kernel's own client: one the medium rule let through on
//! StoreWrite, the installer, whose erase confirmation is typed from it
//! (C5). The model is the CID's product name, trailing blanks dropped.

use super::fault::Fault;
use crate::card::Card;
use crate::protocol::{encode_identify, CAPACITY_LEN, E_MSGSIZE, IDENTIFY_LEN};

pub fn capacity(card: &Card, out: &mut [u8]) -> Result<usize, Fault> {
    let dst = out.get_mut(..CAPACITY_LEN).ok_or(Fault::Refused(E_MSGSIZE))?;
    dst.copy_from_slice(&card.blocks.to_le_bytes());
    Ok(CAPACITY_LEN)
}

pub fn identify(card: &Card, sender: u32, out: &mut [u8]) -> Result<usize, Fault> {
    let dst = out.get_mut(..IDENTIFY_LEN).and_then(|d| <&mut [u8; IDENTIFY_LEN]>::try_from(d).ok());
    let dst = dst.ok_or(Fault::Refused(E_MSGSIZE))?;
    let product = card.cid.product();
    let named = product.iter().rposition(|&b| b != b' ').map_or(0, |last| last + 1);
    let serial = card.cid.serial();
    let serial: &[u8] = if sender == 0 { &[] } else { &serial };
    encode_identify(dst, card.blocks, product.get(..named).unwrap_or(&[]), serial);
    Ok(IDENTIFY_LEN)
}
