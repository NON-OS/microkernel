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

use crate::sys::serial;

pub(super) fn init_entropy() {
    // The kernel drives no RNG device: the CPU's, when it has one, and the
    // entropy capsule's bytes once it runs, each mixed into every seed.
    if crate::crypto::util::rng::has_cpu_entropy() || crate::crypto::util::rng::has_cpu_random() {
        serial::println(b"[NONOS] Entropy: CPU RNG");
    } else {
        serial::println(b"[NONOS] Entropy: no CPU RNG");
    }
}
