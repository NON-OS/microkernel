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

//! The per-boot secret, drawn on its own.
//!
//! This module holds two facilities and only one of them is deliverable
//! today. The slide moves the heap, VM and MMIO windows in the layout record,
//! but the heap maps itself at the `KHEAP_BASE` constant and every other
//! consumer reads the constant too, so applying a slide would leave the record
//! and the running kernel describing different addresses. That is a change to
//! those consumers, not to this module.
//!
//! The nonce has no such dependency. It is a secret, not an address, and
//! everything that uses it reads it through `boot_nonce`. Until this ran, that
//! call failed on every boot and every caller took its `unwrap_or` fallback,
//! which left the stack canary a published function of the cycle counter.

use core::sync::atomic::Ordering;

use super::super::constants::{NONCE_GEN_MULTIPLIER, NONCE_ROTATE_BITS};
use super::super::error::{KaslrError, KaslrResult};
use super::entropy::collect_entropy;
use super::hwrng::has_hardware_rng;
use super::state::BOOT_NONCE;

/// Draw the boot nonce and publish it, answering whether a hardware
/// generator backed the draw.
///
/// The caller reports that answer, because it is the difference between a
/// secret this machine cannot predict and one resting on counter jitter
/// alone. Both are better than the constant that preceded them; only one is
/// worth relying on.
///
/// Idempotent: a nonce already set is left alone, since anything derived from
/// it earlier in the boot would otherwise be holding a stale value.
pub fn seed_boot_nonce() -> KaslrResult<bool> {
    if BOOT_NONCE.load(Ordering::Relaxed) != 0 {
        return Ok(has_hardware_rng());
    }
    let entropy = collect_entropy();
    let nonce = entropy.wrapping_mul(NONCE_GEN_MULTIPLIER).rotate_left(NONCE_ROTATE_BITS);
    if nonce == 0 {
        /*
         * A zero draw is indistinguishable from never having drawn, since
         * that is exactly how `boot_nonce` reports an unset nonce. Refusing
         * is honest; storing it would claim a secret that reads as absent.
         */
        return Err(KaslrError::NotInitialized);
    }
    BOOT_NONCE.store(nonce, Ordering::SeqCst);
    Ok(has_hardware_rng())
}
