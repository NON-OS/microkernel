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

use super::super::entropy::get_entropy64_secure;
use super::super::error::{RngError, RngResult};
use super::fallback::{fill_with_fallback_secure, say_pre_seed, weak_u64};
use super::init::ensure_initialized;
use super::state::GLOBAL_RNG;

#[track_caller]
pub fn get_random_bytes() -> [u8; 32] {
    let mut out = [0u8; 32];

    if ensure_initialized().is_ok() {
        if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
            super::reseed::draw(rng, &mut out);
            return out;
        }
    }

    fill_with_fallback_secure(&mut out);
    out
}

pub fn get_random_bytes_secure() -> RngResult<[u8; 32]> {
    ensure_initialized()?;

    let mut out = [0u8; 32];
    if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
        super::reseed::draw(rng, &mut out);
        return Ok(out);
    }

    Err(RngError::NotInitialized)
}

#[track_caller]
pub fn fill_random_bytes(buf: &mut [u8]) {
    if ensure_initialized().is_ok() {
        if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
            super::reseed::draw(rng, buf);
            return;
        }
    }

    fill_with_fallback_secure(buf);
}

pub fn fill_random_bytes_secure(buf: &mut [u8]) -> RngResult<()> {
    ensure_initialized()?;

    if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
        super::reseed::draw(rng, buf);
        return Ok(());
    }

    Err(RngError::NotInitialized)
}

#[track_caller]
pub fn random_u64() -> u64 {
    if ensure_initialized().is_ok() {
        if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
            return rng.next_u64();
        }
    }

    say_pre_seed(core::panic::Location::caller());
    get_entropy64_secure().unwrap_or_else(|_| weak_u64())
}

pub fn random_u64_secure() -> RngResult<u64> {
    ensure_initialized()?;

    if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
        return Ok(rng.next_u64());
    }

    Err(RngError::NotInitialized)
}

#[track_caller]
pub fn random_u32() -> u32 {
    (random_u64() & 0xFFFF_FFFF) as u32
}

pub fn random_u32_secure() -> RngResult<u32> {
    Ok((random_u64_secure()? & 0xFFFF_FFFF) as u32)
}
