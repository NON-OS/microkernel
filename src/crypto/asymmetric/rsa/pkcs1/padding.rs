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

use crate::crypto::{CryptoError, CryptoResult};
use alloc::vec::Vec;

pub(super) fn pkcs1_pad_type1(data: &[u8], em_len: usize) -> CryptoResult<Vec<u8>> {
    if data.len() > em_len - 11 {
        return Err(CryptoError::InvalidLength);
    }
    let mut em = Vec::with_capacity(em_len);
    em.push(0x00);
    em.push(0x01);
    em.extend(core::iter::repeat_n(0xFF, em_len - data.len() - 3));
    em.push(0x00);
    em.extend_from_slice(data);
    Ok(em)
}
