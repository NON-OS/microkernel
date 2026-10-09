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

use nonos_attest_path::{boot_context, parse_v4, root_words, verify, Kind};
use nox_verify::attest::{words, KIND_BOOTLOADER};
use nox_verify::statements::ATTEST;

use super::error::BootError;

/// The boot epoch every boot slot is enrolled at, the enroll tool's.
pub const BOOT_EPOCH: u64 = 1;
/// The bootloader tree's depth, the enroll tool's.
pub const DEPTH: usize = 8;

/*
 * A bootloader slot, checked as the spawn gate checks a capsule: the context is
 * built here from the measurement, never taken from the trailer; the v4 path
 * must fold to `root`; and the STARK proof must verify over the words of that
 * same context and root. Both must pass.
 */
pub fn membership(
    root: &[u8; 32],
    measurement: &[u8; 32],
    trailer: &[u8],
) -> Result<(), BootError> {
    let ctx = boot_context(measurement, BOOT_EPOCH);
    /*
     * A development image's loader carries the path alone: the same context
     * and the same root, without the STARK proof. Only the dev-attest feature
     * builds this, and only a development kernel turns it on.
     */
    #[cfg(feature = "dev-attest")]
    if trailer.starts_with(&nonos_attest_path::MAGIC) {
        return if verify(root, DEPTH, Kind::Bootloader, &ctx, trailer) {
            Ok(())
        } else {
            Err(BootError::Path)
        };
    }
    let v =
        parse_v4(trailer, Kind::Bootloader, ATTEST.max_proof_bytes).ok_or(BootError::Trailer)?;
    if !verify(root, DEPTH, Kind::Bootloader, &ctx, v.path) {
        return Err(BootError::Path);
    }
    let publics = words(root_words(root).ok_or(BootError::Root)?, &ctx, KIND_BOOTLOADER)
        .ok_or(BootError::Root)?;
    nox_verify::verify(&ATTEST, v.proof, &publics).map_err(|r| BootError::Proof(r.code()))
}
