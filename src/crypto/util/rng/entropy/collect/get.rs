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

use super::super::error::EntropyError;
use super::super::hardware::{cpu_entropy64, cpu_random64, read_cycle_counter};
use super::super::state::ENTROPY_COUNTER;
use crate::drivers::virtio_rng;
use core::sync::atomic::Ordering;

pub fn get_entropy64_secure() -> Result<u64, EntropyError> {
    if virtio_rng::is_available() {
        let mut buf = [0u8; 8];
        if virtio_rng::fill_random(&mut buf).is_ok() {
            return Ok(u64::from_le_bytes(buf));
        }
    }
    if let Some(v) = cpu_entropy64() {
        return Ok(v);
    }
    if let Some(v) = cpu_random64() {
        return Ok(v);
    }
    Err(EntropyError::HardwareFailure)
}

pub fn get_entropy64() -> u64 {
    get_entropy64_secure().unwrap_or_else(|_| super::super::super::entropy_unavailable())
}

pub fn get_tsc_entropy() -> u64 {
    let counter = ENTROPY_COUNTER.fetch_add(1, Ordering::SeqCst);
    counter ^ read_cycle_counter()
}
