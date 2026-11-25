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

//! The personality's attest::verify, standing in: an accepted proof is the
//! kernel's MkLocalVerify, which ends in seal::measured for the caller
//! (src/syscall/microkernel/local_verify.rs, seal/absorb.rs:71-75). Here the
//! caller is the test's supervisor and the measurement the image's BLAKE3.

use super::launch_seal_tests::{kernel_measured, SUPERVISOR};

pub fn verify(_path: &[u8], image: &[u8]) -> Result<(), &'static str> {
    kernel_measured(SUPERVISOR, blake3::hash(image).as_bytes());
    Ok(())
}
