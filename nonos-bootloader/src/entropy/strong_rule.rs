// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The strong sources, the firmware's EFI_RNG_PROTOCOL and the CPU's own
//! generator, and the rule a platform that demands one sets
//! (`arch::cpu::REQUIRE_STRONG_ENTROPY`). The counter and the clock mixed in
//! beside them are guessable, so on their own they never pass that rule.
//! Pure, so it is held on the host (loader_arm_proofs).

/// What the strong sources gave this time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Strong {
    /// EFI_RNG_PROTOCOL filled its buffer.
    pub firmware: bool,
    /// How many 64-bit words the CPU's generator gave.
    pub cpu_draws: usize,
}

/// Whether the draw may stand: always where nothing is demanded, else only
/// when a strong source answered.
pub fn strong_enough(require: bool, s: Strong) -> bool {
    !require || s.firmware || s.cpu_draws > 0
}
